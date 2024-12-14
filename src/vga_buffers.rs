// #[allow(warnings)]
// restrict warnings

// #[repr(u8)]
// repr(u8) attribute, each enum variant is stored as a u8

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// To represents a full color that specifiles foreground and
/// background colors, we create a newtype on top of u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

/// Text Buffer
/// Now we can add structures to represent a screen
/// character and the text buffer
/// Since the field ordering in default structs is undefined in Rust, we need the repr(C) attribute. It guarantees that the struct’s fields
///  are laid out exactly like in a C  struct and thus guarantees the correct field ordering.
/// For the Buffer struct, we use repr(transparent) again to ensure that it has the same memory layout as its single field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

// the VGA text buffer is a 2D array of screenChars
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// the screen struct
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// To actually write to screen, we now create a writer type:
pub struct Write {
    column_position: usize, // ciurrent column position
    color_code: ColorCode,
    // The 'static lifetime specifies that the reference is valid
    // for the whole program run time
    buffer: &'static mut buffer,
}
