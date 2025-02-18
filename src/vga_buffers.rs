use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
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
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// To actually write to screen, we now create a writer type:
pub struct Writer {
    column_position: usize, // ciurrent column position
    color_code: ColorCode,
    // The 'static lifetime specifies that the reference is valid
    // for the whole program run time
    buffer: &'static mut Buffer,
}

/// support Formatting Macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

//. Printing
impl Writer {
    /// Now we can use the Writer to modify the buffer’s characters.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // write last line of buffer
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }
    ///To print whole strings,we can convert them to bytes
    /// and print them one-by-one:
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            // special judge
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    // new line  in the screen
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // clear last line of buffer
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// A Global Interface
/// To provide a global writer that can be used as an interface
/// from other modules, without carrying witer instances around,
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// A print Macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffers::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// try it out!
pub fn _print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_byte(b'H');
    writer.write_str("ello! ");
    writer.write_str("stay with me\n");
    // write! macro
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

#[test_case]
fn test_println_ouput() {
    let s = "Some test string that files on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}
