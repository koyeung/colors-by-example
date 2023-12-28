//! Oklch colorspace.
//!
//! [A perceptual color space for image processing](https://bottosson.github.io/posts/oklab/)

use palette::Oklch;

// min/max oklab.l: 0/1
// min/max oklab_c: 0/0.32249102
// min/max oklab_h: -165.23106/180
// Among ANSI-256 colors:
pub const ANSI256_MAX_CHROMA: f32 = 0.32249102;

/// Quantized Oklch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OklchQuantized {
    /// Quantized perceived lightness
    pub l: u8,
    /// Quantized chroma
    pub chroma: u8,
    /// Quantized hue
    /// -180 to 180 is mapped to 0 to 255
    pub hue: u8,
}

impl From<Oklch> for OklchQuantized {
    fn from(Oklch { l, chroma, hue }: Oklch) -> Self {
        let l = (l * 255.0).clamp(0.0, 255.0) as u8;

        assert!(chroma >= 0.0);
        assert!(chroma <= ANSI256_MAX_CHROMA);
        let chroma = chroma / ANSI256_MAX_CHROMA * 255.0;
        let chroma = chroma.clamp(0.0, 255.0) as u8;

        let hue = if chroma == 0u8 {
            0u8
        } else {
            let hue_degree = hue.into_degrees(); // -180 to 180
            let hue = hue_degree.mul_add(1.0 / 180.0, 1.0) / 2.0 * 255.0;
            hue.clamp(0.0, 255.0) as u8
        };

        // let hue: OklabHue<u8> = hue.into_format();

        OklchQuantized { l, chroma, hue }
    }
}
