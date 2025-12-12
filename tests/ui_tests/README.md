# Bevy Material UI - UI Testing Framework

This directory contains automated UI testing tools for Bevy Material UI components.

## Overview

Since Playwright only works with web browsers, this framework uses:

1. **PyAutoGUI** - For OS-level mouse/keyboard automation (works with any app)
2. **PIL/Pillow** - For screenshot capture
3. **NumPy** - For image comparison (visual regression testing)
4. **Telemetry** - Rust-side component state reporting

## Files

| File | Purpose |
|------|---------|
| `quick_test.py` | Main test runner with telemetry support |
| `visual_diff.py` | Screenshot comparison for visual regression |
| `run_tests.py` | Full test suite with component coverage |
| `analyze_results.py` | Report generator for test results |

## Usage

### Run Quick Tests (with visual regression)

```bash
cd tests/ui_tests
python quick_test.py
```

This will:
1. Start the showcase app with telemetry enabled
2. Run through all components
3. Take screenshots and compare to baselines
4. Print telemetry data from components
5. Generate a visual regression report

### First Run (Creating Baselines)

On first run, no baselines exist. Screenshots are saved as new baselines.

```
✓ checkbox_initial: No baseline found. Saved current image as baseline: checkbox_initial
```

### Subsequent Runs (Regression Testing)

On subsequent runs, screenshots are compared to baselines:

```
✓ checkbox_initial: Matches baseline (0.15% difference)
✗ tabs_tab2_selected: Visual regression detected (5.23% difference)
```

### Update Baselines

After intentional UI changes, update baselines:

```python
from visual_diff import update_baseline
update_baseline("checkbox_initial", "path/to/new_screenshot.png")
```

## Telemetry

Components can report internal state via the `ComponentTelemetry` system:

```rust
// In Rust (showcase.rs)
telemetry.log_event("button_clicked", "primary_button");
telemetry.set_state("slider_value", value.to_string());
```

Python tests read `telemetry.json` to verify component state:

```python
telemetry = read_telemetry()
if telemetry:
    print(telemetry["states"]["slider_value"])
```

## Rust Unit Tests

For component logic testing without UI, use:

```bash
cargo test --test material_component_tests
```

This tests:
- Slider snapping/value calculations
- Tab selection logic
- Navigation state management
- Component builder patterns

## Directory Structure

```
tests/ui_tests/
├── baselines/           # Reference screenshots (git tracked)
├── test_output/         # Generated test artifacts (git ignored)
│   ├── screenshots/
│   ├── diffs/           # Visual diff images
│   └── reports/
├── quick_test.py
├── visual_diff.py
└── README.md
```

## Limitations

Since this isn't Playwright, we cannot:
- Query DOM/UI tree structure
- Wait for specific UI state changes
- Get element properties directly

Workarounds:
- **Telemetry**: Components report state to JSON file
- **Screenshot Comparison**: Detect visual changes
- **Fixed Positions**: Tests use relative coordinates

## Adding New Tests

1. Add test function in `quick_test.py`:
```python
def test_new_component(rect):
    click_relative(rect, x, y)
    capture("new_component_state", rect, check_baseline=True)
```

2. Add to test runner:
```python
all_observations.extend(test_new_component(rect))
```

3. Run once to create baseline, then verify subsequent runs pass.
