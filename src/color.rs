//! Add function to return whether color is gray.

use rgb::RGB8;

pub trait Gray {
    fn is_gray(&self) -> bool;
}

impl Gray for RGB8 {
    /// Check if it is gray color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rgb::RGB8;
    /// # use colors_by_example::color::Gray;
    ///
    /// let gray = RGB8::new(10, 10, 10);
    /// assert!(gray.is_gray());
    ///
    /// let not_gray = RGB8::new(10, 11, 10);
    /// assert!(!not_gray.is_gray());
    /// ```
    fn is_gray(&self) -> bool {
        self.r == self.g && self.g == self.b
    }
}
