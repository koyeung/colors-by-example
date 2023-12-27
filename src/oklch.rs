//! Oklch colorspace.
//!
//! [A perceptual color space for image processing](https://bottosson.github.io/posts/oklab/)

use approx::abs_diff_eq;
use oklab::Oklab;

use std::f32::consts;

/// Color in Oklch is polar form of Oklab.
/// It is denoted by:
/// - L - perceived lightness (same as L in Oklab)
/// - C - chroma
/// - h (in degree) - hue
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Oklch {
    /// L - perceived lightness
    /// should be same value as in oklab
    pub l: f32,

    /// Chroma - Colorfulness
    pub chroma: f32,

    /// Hue (degree) - how much it is perceived as red, yellow, green, blue, etc
    pub hue: f32,

    /// Hue (radian) - how much it is perceived as red, yellow, green, blue, etc
    /// to facilitate conversion to `OklchQuantized`
    hue_radian: f32,
}

// min/max oklab.l: 0/1
// min/max oklab_c: 0/0.32249102
// min/max oklab_h: -165.23106/180
// Among ANSI-256 colors:
pub const ANSI256_MAX_CHROMA: f32 = 0.32249102;

impl From<Oklab> for Oklch {
    /// Convert from Oklab to Oklch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use approx::assert_abs_diff_eq;
    /// use oklab::srgb_to_oklab;
    /// use rgb::RGB8;
    /// # use colors_by_example::oklch::Oklch;
    ///
    /// let srgb = RGB8::new(255, 135, 0);
    /// let oklab = srgb_to_oklab(srgb);
    ///
    /// let oklch = Oklch::from(oklab);
    ///
    /// // cross-check with https://ajalt.github.io/colormath/converter/
    /// // sRGB (0-255) 255, 135, 0
    /// // Oklab 0.74264, 0.10158, 0.15067
    /// // Oklch 0.74264, 0.18171, 56.01118
    /// assert_abs_diff_eq!(oklch.l, oklab.l, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklch.chroma, 0.18171, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklch.hue, 56.01118, epsilon = 0.0001);
    /// ```
    fn from(value: Oklab) -> Self {
        let chroma = (value.a.powi(2) + value.b.powi(2)).sqrt();
        let hue_radian = if abs_diff_eq!(chroma, 0.0) {
            0.0
        } else {
            value.b.atan2(value.a)
        };
        let hue = hue_radian.to_degrees();

        Self {
            l: value.l,
            chroma,
            hue,
            hue_radian,
        }
    }
}

/// Quantized Oklch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OklchQuantized {
    /// Quantized perceived lightness
    pub l: u8,
    /// Quantized chroma
    pub chroma: u8,
    /// Quantized hue
    pub hue: u8,
}

impl From<Oklch> for OklchQuantized {
    fn from(
        Oklch {
            l,
            chroma,
            hue_radian,
            ..
        }: Oklch,
    ) -> Self {
        assert!(l >= 0.0);
        assert!(l <= 255.0);
        let l = (l * 255.0) as u8;

        assert!(chroma >= 0.0);
        let chroma = chroma / ANSI256_MAX_CHROMA * 255.0;
        assert!(chroma <= 255.0);
        let chroma = chroma as u8;

        let hue = {
            if chroma == 0u8 {
                // if chroma is zero, fix hue to 0
                0u8
            } else {
                let hue = hue_radian.mul_add(consts::FRAC_1_PI, 1.0);
                assert!(hue >= 0.0);
                let hue = hue / 2.0 * 255.0;
                assert!(hue <= 255.0);

                hue as u8
            }
        };

        OklchQuantized { l, chroma, hue }
    }
}
