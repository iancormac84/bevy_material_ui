"""
Visual Diff - Screenshot Comparison for UI Testing
===================================================

Compares screenshots to detect visual changes in UI components.
Uses pixel-by-pixel comparison with configurable thresholds.
"""

import subprocess
import sys
from pathlib import Path
from typing import Tuple, Optional
import json

try:
    from PIL import Image, ImageChops, ImageDraw, ImageFilter
    import numpy as np
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "pillow", "numpy"], check=True)
    from PIL import Image, ImageChops, ImageDraw, ImageFilter
    import numpy as np


BASELINE_DIR = Path(__file__).parent / "baselines"
DIFF_DIR = Path(__file__).parent / "test_output" / "diffs"
BASELINE_DIR.mkdir(exist_ok=True)
DIFF_DIR.mkdir(parents=True, exist_ok=True)


class VisualDiff:
    """Compare screenshots for visual regression testing"""
    
    def __init__(self, threshold: float = 0.01, ignore_antialiasing: bool = True):
        """
        Args:
            threshold: Percentage of pixels that can differ (0.0 - 1.0)
            ignore_antialiasing: Apply blur to reduce antialiasing differences
        """
        self.threshold = threshold
        self.ignore_antialiasing = ignore_antialiasing
    
    def compare(self, 
                actual: Path | Image.Image, 
                baseline: Path | Image.Image,
                save_diff: Optional[Path] = None) -> Tuple[bool, float, Optional[Image.Image]]:
        """
        Compare two images.
        
        Returns:
            Tuple of (matches, diff_percentage, diff_image)
        """
        # Load images
        if isinstance(actual, Path):
            actual = Image.open(actual)
        if isinstance(baseline, Path):
            baseline = Image.open(baseline)
        
        # Ensure same size
        if actual.size != baseline.size:
            # Resize actual to match baseline
            actual = actual.resize(baseline.size, Image.Resampling.LANCZOS)
        
        # Convert to RGB
        actual = actual.convert('RGB')
        baseline = baseline.convert('RGB')
        
        # Optional: blur to reduce antialiasing sensitivity
        if self.ignore_antialiasing:
            actual = actual.filter(ImageFilter.GaussianBlur(radius=1))
            baseline = baseline.filter(ImageFilter.GaussianBlur(radius=1))
        
        # Convert to numpy for faster comparison
        actual_arr = np.array(actual)
        baseline_arr = np.array(baseline)
        
        # Calculate difference
        diff = np.abs(actual_arr.astype(int) - baseline_arr.astype(int))
        
        # Pixels are different if any channel differs by more than 10
        different_pixels = np.any(diff > 10, axis=2)
        diff_count = np.sum(different_pixels)
        total_pixels = actual_arr.shape[0] * actual_arr.shape[1]
        diff_percentage = diff_count / total_pixels
        
        # Create diff visualization
        diff_img = None
        if diff_count > 0:
            diff_img = Image.new('RGB', actual.size)
            diff_pixels = diff_img.load()
            actual_pixels = actual.load()
            
            for y in range(actual.size[1]):
                for x in range(actual.size[0]):
                    if different_pixels[y, x]:
                        # Highlight differences in red
                        diff_pixels[x, y] = (255, 0, 0)
                    else:
                        # Dim the matching pixels
                        r, g, b = actual_pixels[x, y]
                        diff_pixels[x, y] = (r // 3, g // 3, b // 3)
            
            if save_diff:
                diff_img.save(save_diff)
        
        matches = diff_percentage <= self.threshold
        return matches, diff_percentage, diff_img
    
    def compare_regions(self,
                       actual: Path | Image.Image,
                       baseline: Path | Image.Image,
                       regions: list[Tuple[str, Tuple[int, int, int, int]]]) -> dict:
        """
        Compare specific regions of images.
        
        Args:
            regions: List of (name, (x1, y1, x2, y2)) tuples
            
        Returns:
            Dict mapping region names to (matches, diff_percentage)
        """
        if isinstance(actual, Path):
            actual = Image.open(actual)
        if isinstance(baseline, Path):
            baseline = Image.open(baseline)
        
        results = {}
        for name, (x1, y1, x2, y2) in regions:
            actual_region = actual.crop((x1, y1, x2, y2))
            baseline_region = baseline.crop((x1, y1, x2, y2))
            
            matches, diff_pct, _ = self.compare(actual_region, baseline_region)
            results[name] = {
                "matches": matches,
                "diff_percentage": diff_pct,
                "region": (x1, y1, x2, y2)
            }
        
        return results


def save_baseline(name: str, image: Path | Image.Image):
    """Save an image as a baseline for future comparisons"""
    if isinstance(image, Path):
        image = Image.open(image)
    
    baseline_path = BASELINE_DIR / f"{name}.png"
    image.save(baseline_path)
    print(f"Saved baseline: {baseline_path}")
    return baseline_path


def load_baseline(name: str) -> Optional[Image.Image]:
    """Load a baseline image"""
    baseline_path = BASELINE_DIR / f"{name}.png"
    if baseline_path.exists():
        return Image.open(baseline_path)
    return None


def compare_with_baseline(name: str, actual: Path | Image.Image, 
                          threshold: float = 0.01) -> dict:
    """
    Compare an image against its baseline.
    
    Returns dict with:
        - has_baseline: bool
        - matches: bool (if has_baseline)
        - diff_percentage: float (if has_baseline)
        - diff_path: Path (if differences found)
        - message: str
    """
    baseline = load_baseline(name)
    
    if baseline is None:
        # No baseline - save this as the new baseline
        save_baseline(name, actual)
        return {
            "has_baseline": False,
            "message": f"No baseline found. Saved current image as baseline: {name}"
        }
    
    differ = VisualDiff(threshold=threshold)
    diff_path = DIFF_DIR / f"{name}_diff.png"
    matches, diff_pct, _ = differ.compare(actual, baseline, save_diff=diff_path)
    
    if matches:
        # Clean up diff file if it matches
        if diff_path.exists():
            diff_path.unlink()
        return {
            "has_baseline": True,
            "matches": True,
            "diff_percentage": diff_pct,
            "message": f"[PASS] {name}: Matches baseline ({diff_pct*100:.2f}% difference)"
        }
    else:
        return {
            "has_baseline": True,
            "matches": False,
            "diff_percentage": diff_pct,
            "diff_path": str(diff_path),
            "message": f"[FAIL] {name}: Visual regression detected ({diff_pct*100:.2f}% difference). Diff saved to {diff_path}"
        }


def update_baseline(name: str, actual: Path | Image.Image):
    """Update a baseline with a new image (after reviewing diff)"""
    return save_baseline(name, actual)


def list_baselines() -> list[str]:
    """List all saved baselines"""
    return [p.stem for p in BASELINE_DIR.glob("*.png")]


def generate_report(results: list[dict]) -> str:
    """Generate a visual regression report"""
    lines = [
        "=" * 60,
        "VISUAL REGRESSION REPORT",
        "=" * 60,
        ""
    ]
    
    passed = sum(1 for r in results if r.get("matches", True))
    failed = sum(1 for r in results if not r.get("matches", True) and r.get("has_baseline", False))
    new = sum(1 for r in results if not r.get("has_baseline", False))
    
    lines.append(f"Summary: {passed} passed, {failed} failed, {new} new baselines")
    lines.append("")
    
    for result in results:
        lines.append(result["message"])
    
    if failed > 0:
        lines.append("")
        lines.append("FAILED TESTS:")
        for result in results:
            if not result.get("matches", True) and result.get("has_baseline", False):
                lines.append(f"  - {result.get('diff_path', 'unknown')}")
    
    return "\n".join(lines)


# Example usage
if __name__ == "__main__":
    print("Visual Diff Tool")
    print("================")
    print(f"Baseline directory: {BASELINE_DIR}")
    print(f"Diff output directory: {DIFF_DIR}")
    print(f"\nExisting baselines: {list_baselines()}")
