//! ANSI-256 colors.

use oklab::{srgb_to_oklab, Oklab};
use rgb::RGB8;

use std::borrow::Borrow;
use std::fmt;

use crate::color::Gray;
use crate::oklch::Oklch;
use crate::palette::Palette;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ansi256Color {
    pub index: u8,
    pub srgb: RGB8,
    pub oklab: Oklab,
    pub oklch: Oklch,
}

impl Ansi256Color {
    /// Construct color info from ANSI-256 index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use approx::assert_abs_diff_eq;
    /// use rgb::RGB8;
    /// # use colors_by_example::ansi256::Ansi256Color;
    ///
    /// let color = Ansi256Color::new(208, 255, 135, 0);
    ///
    /// assert_eq!(color.index, 208);
    /// assert_eq!(color.srgb, RGB8::new(255, 135, 0));
    ///
    /// // cross-check with https://ajalt.github.io/colormath/converter/
    /// // sRGB (0-255) 255, 135, 0
    /// // Oklab 0.74264, 0.10158, 0.15067
    /// // Oklch 0.74264, 0.18171, 56.01118
    /// assert_abs_diff_eq!(color.oklab.l, 0.74264, epsilon = 0.0001);
    /// assert_abs_diff_eq!(color.oklab.a, 0.10158, epsilon = 0.0001);
    /// assert_abs_diff_eq!(color.oklab.b, 0.15067, epsilon = 0.0001);
    ///
    /// assert_abs_diff_eq!(color.oklch.l, 0.74264, epsilon = 0.0001);
    /// assert_abs_diff_eq!(color.oklch.chroma, 0.18171, epsilon = 0.0001);
    /// assert_abs_diff_eq!(color.oklch.hue, 56.01118, epsilon = 0.0001);
    /// ```
    pub fn new(index: u8, r: u8, g: u8, b: u8) -> Self {
        let srgb = RGB8::new(r, g, b);
        let oklab = srgb_to_oklab(srgb);
        let oklch = Oklch::from(oklab);

        Self {
            index,
            srgb,
            oklab,
            oklch,
        }
    }
}

impl fmt::Display for Ansi256Color {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index: {}, RGB8({}, {}, {}), Oklab(l:{}, a:{:.6}, b:{:.6}, chroma:{:.6}, hue:{:.6})",
            self.index,
            self.srgb.r,
            self.srgb.g,
            self.srgb.b,
            self.oklab.l,
            self.oklab.a,
            self.oklab.b,
            self.oklch.chroma,
            self.oklch.hue,
        )
    }
}

impl Gray for Ansi256Color {
    fn is_gray(&self) -> bool {
        self.srgb.is_gray()
    }
}

#[derive(Debug)]
pub struct Ansi256Colors {
    colors: Vec<Ansi256Color>,
}

impl Ansi256Colors {
    /// Construct all ANSI-256 colors with provided palette.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use colors_by_example::ansi256::Ansi256Colors;
    /// use colors_by_example::palette::DEFAULT;
    ///
    /// let colors = Ansi256Colors::new(DEFAULT);
    /// assert_eq!(colors.as_slice().len(), 256);
    /// ```
    pub fn new<T: Borrow<Palette>>(palette: T) -> Self {
        const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];
        // start index of 6x6x6 cube
        const CUBE666_START: u8 = 16;
        // start index of grayscale
        const GRAYSCALE_START: u8 = 232;

        let &Palette(palette) = palette.borrow();

        // chain ranges of:
        // 1. 0 - 15: platform dependent palette
        // 2. 16 - 231: cube 6x6x6 colors
        // 3. 232 - 255: grayscale
        let rgb_channels = palette
            .into_iter()
            .chain(((16u8 - CUBE666_START)..=(231u8 - CUBE666_START)).map(|x| {
                [
                    CUBE[(x / 36) as usize],
                    CUBE[(x / 6 % 6) as usize],
                    CUBE[(x % 6) as usize],
                ]
            }))
            .chain(
                ((232u8 - GRAYSCALE_START)..=(255u8 - GRAYSCALE_START)).map(|x| [x * 10 + 8; 3]),
            );

        Self {
            colors: rgb_channels
                .enumerate()
                .map(|(index, channels)| {
                    Ansi256Color::new(index as u8, channels[0], channels[1], channels[2])
                })
                .collect::<Vec<_>>(),
        }
    }

    /// Return Oklch from ANSI-256 index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use approx::assert_abs_diff_eq;
    /// # use colors_by_example::ansi256::Ansi256Colors;
    ///
    /// let colors = Ansi256Colors::default();
    /// let oklch = colors.oklch_from_index(208);
    ///
    /// // cross-check with https://ajalt.github.io/colormath/converter/
    /// // Oklch 0.74264, 0.18171, 56.01118
    /// assert_abs_diff_eq!(oklch.l, 0.74264, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklch.chroma, 0.18171, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklch.hue, 56.01118, epsilon = 0.0001);
    /// ```
    pub fn oklch_from_index(&self, index: u8) -> Oklch {
        self.colors[index as usize].oklch
    }

    /// Return Oklch from ANSI-256 index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use approx::assert_abs_diff_eq;
    /// # use colors_by_example::ansi256::Ansi256Colors;
    ///
    /// let colors = Ansi256Colors::default();
    /// let oklab = colors.oklab_from_index(208);
    ///
    /// // cross-check with https://ajalt.github.io/colormath/converter/
    /// // Oklab 0.74264, 0.10158, 0.15067
    /// assert_abs_diff_eq!(oklab.l, 0.74264, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklab.a, 0.10158, epsilon = 0.0001);
    /// assert_abs_diff_eq!(oklab.b, 0.15067, epsilon = 0.0001);
    /// ```
    pub fn oklab_from_index(&self, index: u8) -> Oklab {
        self.colors[index as usize].oklab
    }

    /// Return SRGB from ANSI-256 index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rgb::RGB8;
    /// # use colors_by_example::ansi256::Ansi256Colors;
    ///
    /// let colors = Ansi256Colors::default();
    /// let srgb = colors.srgb_from_index(208);
    ///
    /// assert_eq!(srgb, RGB8::new(255, 135, 0));
    /// ```
    pub fn srgb_from_index(&self, index: u8) -> RGB8 {
        self.colors[index as usize].srgb
    }

    /// Extracts a slice containing the entire range of colors.
    pub fn as_slice(&self) -> &[Ansi256Color] {
        &self.colors
    }
}

impl AsRef<Ansi256Colors> for Ansi256Colors {
    fn as_ref(&self) -> &Ansi256Colors {
        self
    }
}

impl Default for Ansi256Colors {
    fn default() -> Self {
        Self::new(Palette::default())
    }
}

/// Indexes of gray in ANSI-256 which is not platform dependent.
///
/// Gray colors are ordered pure black to pure white in increasing intensities.
pub const GRAY_INDEXES: [u8; 26] = [
    16_u8, // black
    232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250,
    251, 252, 253, 254, 255, 231, // white
];

#[cfg(test)]
mod tests {

    use crate::palette::TERMINAL_APP;

    use super::*;

    #[test]
    fn test_ansi256_colors() {
        let colors = Ansi256Colors::new(TERMINAL_APP);

        // color in platform dependent palette
        let color2 = colors.srgb_from_index(2);
        assert_eq!(color2, RGB8::new(0, 166, 0));

        // color in cube 6x6x6
        let color30 = colors.srgb_from_index(30);
        assert_eq!(color30, RGB8::new(0, 135, 135));

        // color in grayscale range
        let color253 = colors.srgb_from_index(253);
        assert_eq!(color253, RGB8::new(218, 218, 218));
    }

    #[test]
    fn test_ansi256_grayscale() {
        let colors = Ansi256Colors::default();

        let black = colors.srgb_from_index(GRAY_INDEXES[0]);
        assert_eq!(black, RGB8::new(0, 0, 0));

        let white = colors.srgb_from_index(GRAY_INDEXES[25]);
        assert_eq!(white, RGB8::new(255, 255, 255));

        for i in 1..25 {
            let c = colors.srgb_from_index(GRAY_INDEXES[i]);
            assert_eq!(c.r, c.g);
            assert_eq!(c.g, c.b);
        }
    }
}
