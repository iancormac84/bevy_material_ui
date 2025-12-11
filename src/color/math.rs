//! Math utilities for color space conversions
//!
//! These functions implement the mathematical operations needed for
//! converting between color spaces (sRGB, Linear RGB, XYZ, L*a*b*, CAM16).

use std::f64::consts::PI;

/// Linear interpolation between two values
pub(crate) fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

/// Convert radians to degrees
pub(crate) fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Sanitize degrees to be within [0, 360)
pub(crate) fn sanitize_degrees(degrees: f64) -> f64 {
    let mut result = degrees % 360.0;
    if result < 0.0 {
        result += 360.0;
    }
    result
}

/// sRGB color component to linear RGB
pub(crate) fn linearize(rgb_component: f64) -> f64 {
    if rgb_component <= 0.04045 {
        rgb_component / 12.92
    } else {
        ((rgb_component + 0.055) / 1.055).powf(2.4)
    }
}

/// Linear RGB component to sRGB
pub(crate) fn delinearize(linear_component: f64) -> f64 {
    if linear_component <= 0.0031308 {
        linear_component * 12.92
    } else {
        1.055 * linear_component.powf(1.0 / 2.4) - 0.055
    }
}

/// Convert a value in [0, 1] to an 8-bit integer [0, 255]
pub(crate) fn to_8bit(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Convert an 8-bit integer [0, 255] to [0, 1]
pub(crate) fn from_8bit(value: u8) -> f64 {
    value as f64 / 255.0
}

/// Extract red component from ARGB integer
pub(crate) fn red_from_argb(argb: u32) -> u8 {
    ((argb >> 16) & 0xFF) as u8
}

/// Extract green component from ARGB integer
pub(crate) fn green_from_argb(argb: u32) -> u8 {
    ((argb >> 8) & 0xFF) as u8
}

/// Extract blue component from ARGB integer
pub(crate) fn blue_from_argb(argb: u32) -> u8 {
    (argb & 0xFF) as u8
}

/// Create ARGB integer from components
pub(crate) fn argb_from_components(alpha: u8, red: u8, green: u8, blue: u8) -> u32 {
    ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

/// Create ARGB integer from RGB components (alpha = 255)
pub(crate) fn argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
    argb_from_components(255, red, green, blue)
}

/// Convert ARGB to linear RGB
pub(crate) fn linear_rgb_from_argb(argb: u32) -> [f64; 3] {
    let r = linearize(from_8bit(red_from_argb(argb)));
    let g = linearize(from_8bit(green_from_argb(argb)));
    let b = linearize(from_8bit(blue_from_argb(argb)));
    [r, g, b]
}

/// sRGB to XYZ transformation matrix
pub(crate) const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

/// Matrix-vector multiplication
pub(crate) fn matrix_multiply(matrix: [[f64; 3]; 3], vector: [f64; 3]) -> [f64; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}

/// Convert linear RGB to XYZ
pub(crate) fn xyz_from_linear_rgb(r: f64, g: f64, b: f64) -> [f64; 3] {
    matrix_multiply(SRGB_TO_XYZ, [r, g, b])
}

/// D65 white point X
pub(crate) const WHITE_POINT_D65_X: f64 = 95.047;
/// D65 white point Y
pub(crate) const WHITE_POINT_D65_Y: f64 = 100.0;
/// D65 white point Z
pub(crate) const WHITE_POINT_D65_Z: f64 = 108.883;

/// Calculate Y (luminance) from ARGB
pub(crate) fn y_from_argb(argb: u32) -> f64 {
    let [r, g, b] = linear_rgb_from_argb(argb);
    SRGB_TO_XYZ[1][0] * r + SRGB_TO_XYZ[1][1] * g + SRGB_TO_XYZ[1][2] * b
}

/// Convert Y (luminance) to L* (perceptual lightness)
/// Y should be in range [0, 1] where 1 is reference white
pub(crate) fn lstar_from_y(y: f64) -> f64 {
    if y <= 216.0 / 24389.0 {
        y * (24389.0 / 27.0)
    } else {
        116.0 * y.cbrt() - 16.0
    }
}

/// Convert L* to Y
pub(crate) fn y_from_lstar(lstar: f64) -> f64 {
    if lstar <= 8.0 {
        lstar * 27.0 / 24389.0
    } else {
        ((lstar + 16.0) / 116.0).powi(3)
    }
}

/// Calculate L* (tone) from ARGB
pub(crate) fn lstar_from_argb(argb: u32) -> f64 {
    // y_from_argb returns value in [0, 1], which is correct for lstar_from_y
    lstar_from_y(y_from_argb(argb))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linearize_delinearize() {
        // Test roundtrip
        for i in 0..=255 {
            let srgb = i as f64 / 255.0;
            let linear = linearize(srgb);
            let back = delinearize(linear);
            assert!((srgb - back).abs() < 0.001, "Failed at {i}");
        }
    }

    #[test]
    fn test_argb_components() {
        let argb = 0xFF8040FF;
        assert_eq!(red_from_argb(argb), 128);
        assert_eq!(green_from_argb(argb), 64);
        assert_eq!(blue_from_argb(argb), 255);
    }

    #[test]
    fn test_argb_from_components() {
        let argb = argb_from_components(255, 128, 64, 255);
        assert_eq!(argb, 0xFF8040FF);
    }

    #[test]
    fn test_sanitize_degrees() {
        assert!((sanitize_degrees(0.0) - 0.0).abs() < 0.001);
        assert!((sanitize_degrees(360.0) - 0.0).abs() < 0.001);
        assert!((sanitize_degrees(-90.0) - 270.0).abs() < 0.001);
        assert!((sanitize_degrees(450.0) - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_lstar_from_y() {
        // Black
        assert!((lstar_from_y(0.0) - 0.0).abs() < 0.001);
        // White
        assert!((lstar_from_y(1.0) - 100.0).abs() < 0.001);
        // Mid gray
        assert!(lstar_from_y(0.18) > 40.0 && lstar_from_y(0.18) < 60.0);
    }

    #[test]
    fn test_y_from_lstar() {
        // Roundtrip
        for l in [0.0, 25.0, 50.0, 75.0, 100.0] {
            let y = y_from_lstar(l);
            let back = lstar_from_y(y);
            assert!((l - back).abs() < 0.001, "Failed at L*={l}");
        }
    }
}
