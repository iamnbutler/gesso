//! Color construction helpers for motif.
//!
//! [`Srgba`] stores components as `f32` in the range `[0.0, 1.0]`. These
//! helpers let you construct colors from common input formats without
//! manually managing the scaling from `[0, 255]` to `[0.0, 1.0]`.

use palette::Srgba;

/// Create an opaque color from floating-point `[0.0, 1.0]` components.
///
/// Shorter alias for `Srgba::new(r, g, b, 1.0)`.
///
/// # Example
/// ```
/// let red = motif_core::rgb(1.0, 0.0, 0.0);
/// ```
pub fn rgb(r: f32, g: f32, b: f32) -> Srgba {
    Srgba::new(r, g, b, 1.0)
}

/// Create a color from floating-point `[0.0, 1.0]` components including alpha.
///
/// Shorter alias for `Srgba::new(r, g, b, a)`.
///
/// # Example
/// ```
/// let translucent_blue = motif_core::rgba(0.0, 0.0, 1.0, 0.5);
/// ```
pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Srgba {
    Srgba::new(r, g, b, a)
}

/// Create an opaque gray from a single `[0.0, 1.0]` lightness value.
///
/// `gray(0.0)` is black; `gray(1.0)` is white.
///
/// # Example
/// ```
/// let mid_gray = motif_core::gray(0.5);
/// ```
pub fn gray(value: f32) -> Srgba {
    Srgba::new(value, value, value, 1.0)
}

/// Create an opaque color from `u8` components in the range `[0, 255]`.
///
/// # Example
/// ```
/// let coral = motif_core::rgb_u8(255, 127, 80);
/// ```
pub fn rgb_u8(r: u8, g: u8, b: u8) -> Srgba {
    Srgba::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        1.0,
    )
}

/// Create a color from `u8` components in the range `[0, 255]` including alpha.
///
/// # Example
/// ```
/// let semi_transparent = motif_core::rgba_u8(0, 0, 0, 128);
/// ```
pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Srgba {
    Srgba::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

/// Return a copy of `color` with its alpha component replaced by `alpha`.
///
/// # Example
/// ```
/// let red = motif_core::rgb(1.0, 0.0, 0.0);
/// let faint_red = motif_core::with_alpha(red, 0.1);
/// assert!((faint_red.alpha - 0.1).abs() < 1e-6);
/// ```
pub fn with_alpha(color: Srgba, alpha: f32) -> Srgba {
    let mut c = color;
    c.alpha = alpha;
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_has_full_alpha() {
        let c = rgb(1.0, 0.5, 0.0);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 0.5);
        assert_eq!(c.blue, 0.0);
        assert_eq!(c.alpha, 1.0);
    }

    #[test]
    fn rgba_preserves_all_components() {
        let c = rgba(0.2, 0.4, 0.6, 0.8);
        assert!((c.red - 0.2).abs() < 1e-6);
        assert!((c.green - 0.4).abs() < 1e-6);
        assert!((c.blue - 0.6).abs() < 1e-6);
        assert!((c.alpha - 0.8).abs() < 1e-6);
    }

    #[test]
    fn gray_has_equal_rgb_and_full_alpha() {
        let c = gray(0.5);
        assert_eq!(c.red, 0.5);
        assert_eq!(c.green, 0.5);
        assert_eq!(c.blue, 0.5);
        assert_eq!(c.alpha, 1.0);
    }

    #[test]
    fn gray_extremes() {
        let black = gray(0.0);
        assert_eq!(black.red, 0.0);
        assert_eq!(black.green, 0.0);
        assert_eq!(black.blue, 0.0);

        let white = gray(1.0);
        assert_eq!(white.red, 1.0);
        assert_eq!(white.green, 1.0);
        assert_eq!(white.blue, 1.0);
    }

    #[test]
    fn rgb_u8_full_value_maps_to_one() {
        let c = rgb_u8(255, 0, 0);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 0.0);
        assert_eq!(c.blue, 0.0);
        assert_eq!(c.alpha, 1.0);
    }

    #[test]
    fn rgb_u8_zero_maps_to_zero() {
        let c = rgb_u8(0, 0, 0);
        assert_eq!(c.red, 0.0);
        assert_eq!(c.green, 0.0);
        assert_eq!(c.blue, 0.0);
    }

    #[test]
    fn rgb_u8_mid_values() {
        let c = rgb_u8(128, 64, 32);
        assert!((c.red - 128.0 / 255.0).abs() < 1e-6);
        assert!((c.green - 64.0 / 255.0).abs() < 1e-6);
        assert!((c.blue - 32.0 / 255.0).abs() < 1e-6);
        assert_eq!(c.alpha, 1.0);
    }

    #[test]
    fn rgba_u8_full_and_zero_components() {
        let c = rgba_u8(255, 0, 255, 128);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.green, 0.0);
        assert_eq!(c.blue, 1.0);
        assert!((c.alpha - 128.0 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn rgba_u8_fully_transparent() {
        let c = rgba_u8(255, 255, 255, 0);
        assert_eq!(c.red, 1.0);
        assert_eq!(c.alpha, 0.0);
    }

    #[test]
    fn with_alpha_replaces_alpha_only() {
        let red = rgb(1.0, 0.0, 0.0);
        let faint_red = with_alpha(red, 0.1);
        assert_eq!(faint_red.red, 1.0);
        assert_eq!(faint_red.green, 0.0);
        assert_eq!(faint_red.blue, 0.0);
        assert!((faint_red.alpha - 0.1).abs() < 1e-6);
    }

    #[test]
    fn with_alpha_zero_makes_transparent() {
        let color = rgb(0.5, 0.5, 0.5);
        let transparent = with_alpha(color, 0.0);
        assert_eq!(transparent.alpha, 0.0);
        // RGB is unchanged
        assert_eq!(transparent.red, 0.5);
    }
}
