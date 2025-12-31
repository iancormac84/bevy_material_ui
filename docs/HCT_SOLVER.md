# HCT Solver - Material Design 3 Color System

This module implements the complete Material Design 3 HCT (Hue-Chroma-Tone) color solver with accurate gamut mapping.

## Overview

HCT is a perceptually accurate color space that combines:
- **Hue** (0-360°): The color angle on the color wheel, from CAM16
- **Chroma** (0-~150): How colorful/saturated the color is, from CAM16  
- **Tone** (0-100): Perceptual lightness, from L*a*b*

The key advantage of HCT is that tone differences directly correspond to contrast:
- Tone difference of 50+ ensures WCAG 4.5:1 contrast (sufficient for small text)
- Tone difference of 40+ ensures WCAG 3:1 contrast (sufficient for large text)

## Implementation

This is a complete Rust port of the Material Design 3 HctSolver algorithm from the reference Material Components implementation.

### Key Features

1. **Newton's Method Iteration**: Finds exact colors in CAM16 space using 5 rounds of Newton's method with convergence threshold of 0.002
2. **Gamut Boundary Detection**: Uses 255 critical planes to accurately map colors to sRGB gamut boundaries
3. **Bisection Algorithm**: Falls back to bisection on gamut edges when Newton's method fails
4. **CAM16 Color Appearance Model**: Full chromatic adaptation with proper viewing conditions

### Algorithm Components

#### 1. Critical Planes (255 values)
Pre-calculated Y values (luminance planes) used to efficiently find gamut boundaries. These planes represent key luminance levels where the sRGB gamut shape changes significantly.

#### 2. Newton's Method (`find_result_by_j`)
Iteratively solves for RGB values that match the desired HCT values:
- Starts from a J (lightness) estimate
- Performs 5 iterations using Newton's method
- Converges when |fn(j) - y| < 0.002
- Returns None if color is outside gamut

#### 3. Bisection Algorithm (`bisect_to_limit`)
When Newton's method fails (color outside gamut), finds the closest in-gamut color:
- Finds segment containing target hue on the Y plane (`bisect_to_segment`)
- Uses binary search with critical planes (max 8 iterations)
- Returns the midpoint of the closest gamut edge segment

#### 4. CAM16 Functions
- `chromatic_adaptation`: Apply CAM16 chromatic adaptation (component^0.42 transform)
- `inverse_chromatic_adaptation`: Reverse chromatic adaptation  
- `hue_of`: Calculate CAM16 hue angle from linear RGB

#### 5. Color Space Transforms
Three transformation matrices:
- `SCALED_DISCOUNT_FROM_LINRGB`: Linear RGB → Scaled discount RGB  
- `LINRGB_FROM_SCALED_DISCOUNT`: Scaled discount RGB → Linear RGB
- `Y_FROM_LINRGB`: Linear RGB → Y (luminance)

## Usage

The HctSolver is automatically used when creating HCT colors:

```rust
use bevy_material_ui::color::Hct;

// Create an HCT color (hue, chroma, tone)
let green = Hct::new(140.0, 60.0, 50.0);
println!("ARGB: #{:08X}", green.to_argb()); // #FF278900

// The solver automatically handles gamut mapping
// Requested chroma of 60 becomes actual chroma of ~14
// because green at tone 50 has limited chroma in sRGB
println!("Actual chroma: {:.2}", green.chroma()); // 13.99

// Generate a tonal palette
for tone in [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
    let color = Hct::new(140.0, 60.0, tone as f64);
    println!("Tone {}: #{:08X}", tone, color.to_argb());
}
```

## Differences from Simple HSL Solver

The previous simplified HSL-based solver had several issues:

### Old Implementation (Broken)
```rust
// ❌ Uses HSL color space (not perceptually uniform)
// ❌ No CAM16 iteration
// ❌ Improper gamut handling (especially for greens)
// ❌ Samples 100 saturation values (slow and inaccurate)

for sat_step in 0..=100 {
    let saturation = sat_step as f64 / 100.0;
    // HSL to RGB conversion...
    // Find best match by comparing hue, chroma, tone
}
```

### New Implementation (Correct)
```rust
// ✅ Uses CAM16 perceptual color space
// ✅ Newton's method iteration (5 rounds, ε=0.002)
// ✅ Proper gamut mapping with 255 critical planes
// ✅ Fast and accurate

fn solve_to_argb(hue: f64, chroma: f64, tone: f64) -> u32 {
    let y = y_from_lstar(tone);
    
    // Try Newton's method first
    if let Some(exact) = find_result_by_j(hue_radians, chroma, y) {
        return exact;
    }
    
    // Fall back to gamut boundary bisection
    let linrgb = bisect_to_limit(y, hue_radians);
    argb_from_linrgb(linrgb)
}
```

## Why This Matters

### Problem: Green Colors
The sRGB gamut is narrowest in the green region. At tone 50, pure green can only achieve about chroma 14, not 60. The old solver would:
1. Generate incorrect colors (too dark or wrong hue)
2. Fail to find colors near gamut boundaries
3. Produce inconsistent results across the hue range

### Solution: Accurate Gamut Mapping
The new solver:
1. Correctly finds maximum chroma for each hue/tone combination
2. Gracefully clamps requested chroma to achievable values
3. Maintains hue and tone accuracy while maximizing chroma

## Viewing Conditions

The solver uses standard sRGB viewing conditions:
- **White Point**: D65 (6504K daylight)
- **Adapting Luminance**: 11.72 cd/m² (~200 lux, typical office)
- **Background**: 20% gray
- **Surround**: Average (1.0)
- **Adaptation**: Full (D = 1.0)

## Mathematical Details

### CAM16 Chromatic Adaptation
```rust
fn chromatic_adaptation(component: f64) -> f64 {
    let af = component.abs().powf(0.42);
    component.signum() * 400.0 * af / (af + 27.13)
}
```

### Y ↔ L* Conversion
Uses CIE L*a*b* formulas with Y in [0, 100] range:
```rust
// L* → Y
fn y_from_lstar(lstar: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft = (lstar + 16.0) / 116.0;
    let ft3 = ft * ft * ft;
    
    100.0 * if ft3 > e { ft3 } else { (116.0 * ft - 16.0) / kappa }
}
```

### Gamut Boundary Check
```rust
fn is_bounded(x: f64) -> bool {
    0.0 <= x && x <= 100.0  // Linear RGB in [0, 100] range
}
```

## Testing

Run the color tests to verify correct operation:

```bash
cargo test color --lib

# Test specifically with green colors
cargo run --example test_green_colors
cargo run --example debug_hctsolver
```

Example output for greens:
```
Material Green (H:140, C:60, T:50):
  Result: Hue=139.34, Chroma=13.99, Tone=49.89
  ARGB: #FF278900

Green Tonal Palette (H:140, C:60):
  Tone  10: #FF042100 (actual chroma: 5.55)
  Tone  20: #FF0B3900 (actual chroma: 7.89)
  Tone  30: #FF145200 (actual chroma: 10.02)
  Tone  40: #FF1D6D00 (actual chroma: 12.08)
  Tone  50: #FF278900 (actual chroma: 13.99)
  Tone  60: #FF31A700 (actual chroma: 15.88)
  Tone  70: #FF3BC500 (actual chroma: 17.62)
  Tone  80: #FF4EE319 (actual chroma: 18.67)
```

## References

- [Material Design 3 - The Science of Color & Design](https://m3.material.io/blog/science-of-color-design)
- Reference implementation: `HctSolver.java` in the Material Components source tree.
- [CAM16 Color Appearance Model](https://doi.org/10.1002/col.22131)
- [CIE L*a*b* Color Space](https://en.wikipedia.org/wiki/CIELAB_color_space)

## License

This implementation is part of bevy_material_ui, licensed under the same terms as the parent project.

The algorithm is based on Material Design 3 reference implementation which is licensed under Apache 2.0.
