//! Platform dependent 4-bit color codes.
//!
//! [3-bit and 4-bit Colors](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit)

/// System defined colors.
///
/// Different platform has different palette.
/// Actual values are configurable in terminal emulators.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Base16(pub [[u8; 3]; 16]);

impl Base16 {
    pub fn new() -> Self {
        Base16::default()
    }
}

impl Default for Base16 {
    fn default() -> Self {
        DEFAULT
    }
}

#[cfg(target_os = "macos")]
pub use TERMINAL_APP as DEFAULT;

#[cfg(target_os = "windows")]
pub use WIN10_CONSOLE as DEFAULT;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use WIN10_CONSOLE as DEFAULT;

// Values from Terminal.app
pub const TERMINAL_APP: Base16 = Base16([
    // standard colors
    [0, 0, 0],       // Black
    [153, 0, 0],     // Red
    [0, 166, 0],     // Green
    [153, 153, 0],   // Yellow
    [0, 0, 178],     // Blue
    [178, 0, 178],   // Magenta
    [0, 166, 178],   // Cyan
    [191, 191, 191], // White
    // high intensity colors
    [102, 102, 102], // Bright Black (Gray)
    [230, 0, 0],     // Bright Red
    [0, 217, 0],     // Bright Green
    [230, 230, 0],   // Bright Yellow
    [0, 0, 255],     // Bright Blue
    [230, 0, 230],   // Bright Magenta
    [0, 230, 230],   // Bright Cyan
    [230, 230, 230], // Bright White
]);

// Values from: https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
pub const WIN10_CONSOLE: Base16 = Base16([
    // standard colors
    [12, 12, 12],    // Black
    [197, 15, 31],   // Red
    [19, 161, 14],   // Green
    [193, 156, 0],   // Yellow
    [0, 55, 218],    //Blue
    [136, 23, 152],  //Magenta
    [58, 150, 221],  //Cyan
    [204, 204, 204], //White
    // high intensity colors
    [118, 118, 118], // Bright Black (Gray)
    [231, 72, 86],   // Bright Red
    [22, 198, 12],   // Bright Green
    [249, 241, 165], //Bright Yellow
    [59, 120, 255],  //Bright Blue
    [180, 0, 158],   // Bright Magenta
    [97, 214, 214],  //Bright Cyan
    [242, 242, 242], //Bright White
]);

// Values from: https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
pub const XTERM: Base16 = Base16([
    // standard colors
    [0, 0, 0],       // Black
    [205, 0, 0],     // Red
    [0, 205, 0],     // Green
    [205, 205, 0],   // Yellow
    [0, 0, 238],     // Blue
    [205, 0, 205],   // Magenta
    [0, 205, 205],   // Cyan
    [229, 229, 229], // White
    // high intensity colors
    [127, 127, 127], // Bright Black (Gray)
    [255, 0, 0],     // Bright Red
    [0, 255, 0],     // Bright Green
    [255, 255, 0],   // Bright Yellow
    [92, 92, 255],   // Bright Blue
    [255, 0, 255],   // Bright Magenta
    [0, 255, 255],   // Bright Cyan
    [255, 255, 255], // Bright White
]);
