/**
 *  VGA text mode module.
 *  Prints things on screen by writting to memory I/O buffer.
 */

use volatile::Volatile;

#[allow(dead_code)]  // disable warnings for unused variants
#[repr(u8)]          // store each variant as `u8`
pub enum Color {
    /* Available colors for VGA text mode
        (8 to 15 are lighter variants sometimes exclusive to foreground). */
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

#[repr(transparent)]    // ensure same data layout as `u8`
#[derive(Clone, Copy)]  // allow ColorCode variables to be copied
struct ColorCode(u8);

impl ColorCode {
    /* VGA text mode color code byte consisting of 3 parts:
        0 to 3 -> foreground; 4 to 6 -> background; 7 -> blink bit. */

    fn new(fg: Color, bg: Color, blink: bool) -> ColorCode {
        /* Create a new ColorCode by setting the respective bits in byte.
            If a bg value >= 8 is given, ignore blink bit instead. */
        let _bg = bg as u8;
        let mut byte = (_bg << 4) + (fg as u8);
        if (_bg as u8) < 8 {
            byte += (blink as u8) << 7;
        }
        ColorCode(byte)
    }
}

#[repr(C)]              // guarantee correct field ordering
#[derive(Clone, Copy)]  // Copy is needed for the Volatile type
struct ScreenChar {
    /* Two-byte (sequential) structure
        representing a char in VGA text mode. */
    ascii_character: u8,    // the character in ASCII value
    color_code: ColorCode,  // the color code byte
}

// Buffer screen dimensions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]  // ensure same memory layout as of a single field
struct Buffer {
    /* Matrix buffer that must point to
        VGA text mode's memory I/O location. */
    chars: [[Volatile<ScreenChar>;
            BUFFER_WIDTH]; BUFFER_HEIGHT],  // the underlying byte matrix
}

pub struct Writer {
    /* Writer class; write (colored) bytes
        and strings to VGA text buffer. */
    column_position: usize,  // horizontal index on buffer
    color_code: ColorCode,   // color code of the following bytes
    buffer: *mut Buffer,     // pointer to text buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        /* Write a single byte to buffer. */

        match byte {
            // Write a newline if '\n'
            b'\n' => self.new_line(),

            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    // Go to next line if screen's end reached
                    self.new_line();
                }
                // Pick last row and current column
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // Create a ScreenChar from given byte and current color
                let color_code = self.color_code;
                let screen_char = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                unsafe {
                    // Write (two-byte) char to memory I/O buffer
                    // (`write` must be used as it is of the Volatile type)
                    (*self.buffer).chars[row][col].write(screen_char);
                }
                // Increment column index
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        /* Write given string to buffer, byte by byte. */
        for byte in s.bytes() {
            match byte {
                // Char in printable range or '\n'
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Not part of printable ASCII range, so print a ■
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {/* TODO */}
}

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Black, Color::LightGray, true),
        buffer: 0xb8000 as *mut Buffer,
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
