//! Color with quantized values in colorspaces.

use palette::Srgb;

use std::ops::Index;

use crate::ansi256::{Ansi256Colors, GRAY_INDEXES};
use crate::color::Gray;
use crate::oklch_quantize::OklchQuantized;

/// Ansi-256 colors in quantized form.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ansi256QuantizedColors {
    colors: Vec<Ansi256QuantizedColor>,
}

impl Ansi256QuantizedColors {
    /// Construct quantized ANSI-256 color info.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use palette::Srgb;
    /// # use colors_by_example::ansi256_quantize::{Ansi256QuantizedColors, Ansi256QuantizedColor};
    /// # use colors_by_example::ansi256::Ansi256Colors;
    /// let colors = Ansi256QuantizedColors::new(Ansi256Colors::default());
    ///
    /// let color17 = colors.color_from_index(17);
    /// assert_eq!(
    ///     color17,
    ///     Ansi256QuantizedColor {
    ///         index: 17,
    ///         srgb: Srgb::new(0u8, 0, 95),
    ///         l: 55,
    ///         hue: 59,
    ///         chroma: 120,
    ///         lightness_level: 3
    ///     }
    /// );
    /// ```
    pub fn new<T: AsRef<Ansi256Colors>>(ansi256_colors: T) -> Self {
        let ansi256_colors = ansi256_colors.as_ref().as_slice();

        let oklch_quantized = ansi256_colors
            .iter()
            .map(|&color| OklchQuantized::from(color.oklch))
            .collect::<Vec<_>>();

        let oklch_quantized_with_lightness = associate_lightness_levels(oklch_quantized);

        let colors = ansi256_colors
            .iter()
            .zip(oklch_quantized_with_lightness.iter())
            .map(
                |(&ansi256_color, &(oklch, lightness_level))| Ansi256QuantizedColor {
                    index: ansi256_color.index,
                    srgb: ansi256_color.srgb,
                    l: oklch.l,
                    chroma: oklch.chroma,
                    hue: oklch.hue,
                    lightness_level,
                },
            )
            .collect::<Vec<_>>();

        Self { colors }
    }

    /// Return quantized color information from ANSI-256 index.
    pub fn color_from_index(&self, index: u8) -> Ansi256QuantizedColor {
        self.colors[index as usize]
    }

    /// Check if color of index is gray.
    pub fn is_gray_from_index(&self, index: u8) -> bool {
        self.color_from_index(index).is_gray()
    }

    /// Extracts a slice containing the entire range of colors.
    pub fn as_slice(&self) -> &[Ansi256QuantizedColor] {
        &self.colors
    }
}

impl Default for Ansi256QuantizedColors {
    fn default() -> Self {
        Self::new(Ansi256Colors::default())
    }
}

/// ANSI-256 color in quantized form with lightness level.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ansi256QuantizedColor {
    /// ANSI-256
    pub index: u8,

    /// SRGB
    pub srgb: Srgb<u8>,

    /// Quantized perceived lightness
    pub l: u8,
    /// Quantized chroma
    pub chroma: u8,
    /// Quantized hue
    pub hue: u8,

    /// lightness level according to grays in ANSI-256 colors
    pub lightness_level: u8,
}

impl Gray for Ansi256QuantizedColor {
    fn is_gray(&self) -> bool {
        self.srgb.is_gray()
    }
}

fn associate_lightness_levels<T>(colors: T) -> Vec<(OklchQuantized, u8)>
where
    T: IntoIterator<Item = OklchQuantized> + Index<usize, Output = OklchQuantized>,
{
    let lightness_bounds = lightness_level_bounds(&colors, &GRAY_INDEXES);

    colors
        .into_iter()
        .map(|color| {
            // level would be lowerest bound that is >= the color's lightness
            let level = lightness_bounds
                .iter()
                .position(|&bound| color.l <= bound)
                .unwrap();

            (color, level as u8)
        })
        .collect::<Vec<_>>()
}

/// Return vector of increasing lightness values (quantized).
///
/// Input `gray-indexes` should be increasing on lightness.
///
/// # Panic
/// Would panic if `gray_indexes` is out of `colors`' bounds.
fn lightness_level_bounds<T: Index<usize, Output = OklchQuantized>>(
    colors: &T,
    gray_indexes: &[u8],
) -> Vec<u8> {
    // extract grays with increasing brightness (from black to white)
    let grays = gray_indexes
        .iter()
        .map(|&i| colors[i as usize])
        .collect::<Vec<_>>();

    // lightness bounds would be mid-point between adjacent grays
    let mut gray_lightness = grays
        .windows(2)
        .map(|x| {
            u8::try_from(((x[0].l as u16) + (x[1].l as u16)) >> 1)
                .expect("mid-point of two l should also be u8")
        })
        .collect::<Vec<_>>();

    // the last gray would have lightness from itself
    let last_gray = grays.last().unwrap();
    gray_lightness.push(last_gray.l);

    gray_lightness
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quantize_colors() {
        let quantized = Ansi256QuantizedColors::default();

        assert_eq!(quantized.as_slice().len(), 256);

        let black = quantized.color_from_index(16);
        assert_eq!(
            black,
            Ansi256QuantizedColor {
                index: 16,
                srgb: Srgb::new(0u8, 0, 0),
                l: 0,
                hue: 0,
                chroma: 0,
                lightness_level: 0
            }
        );

        let gray1 = quantized.color_from_index(232);
        assert_eq!(
            gray1,
            Ansi256QuantizedColor {
                index: 232,
                srgb: Srgb::new(8u8, 8, 8),
                l: 34,
                hue: 0,
                chroma: 0,
                lightness_level: 1
            }
        );

        let gray2 = quantized.color_from_index(233);
        assert_eq!(
            gray2,
            Ansi256QuantizedColor {
                index: 233,
                srgb: Srgb::new(18u8, 18, 18),
                l: 46,
                hue: 0,
                chroma: 0,
                lightness_level: 2
            }
        );

        let color17 = quantized.color_from_index(17);
        assert_eq!(
            color17,
            Ansi256QuantizedColor {
                index: 17,
                srgb: Srgb::new(0u8, 0, 95),
                l: 55,
                hue: 59,
                chroma: 120,
                lightness_level: 3
            }
        );

        let gray3 = quantized.color_from_index(234);
        assert_eq!(
            gray3,
            Ansi256QuantizedColor {
                index: 234,
                srgb: Srgb::new(28u8, 28, 28),
                l: 57,
                hue: 0,
                chroma: 0,
                lightness_level: 3
            }
        );

        let white = quantized.color_from_index(231);
        assert_eq!(
            white,
            Ansi256QuantizedColor {
                index: 231,
                srgb: Srgb::new(255u8, 255, 255),
                l: 255,
                hue: 0,
                chroma: 0,
                lightness_level: 25
            }
        );

        for (idx, c) in quantized.as_slice().iter().enumerate() {
            println!("{idx} {c:?}")
        }
    }
}
