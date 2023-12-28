//! Add function to return whether color is gray.

use palette::Srgb;

pub trait Gray {
    fn is_gray(&self) -> bool;
}

impl Gray for Srgb<u8> {
    /// Check if it is gray color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use palette::Srgb;
    /// # use colors_by_example::color::Gray;
    ///
    /// let gray = Srgb::new(10, 10, 10);
    /// assert!(gray.is_gray());
    ///
    /// let not_gray = Srgb::new(10, 11, 10);
    /// assert!(!not_gray.is_gray());
    /// ```
    fn is_gray(&self) -> bool {
        self.red == self.green && self.green == self.blue
    }
}
