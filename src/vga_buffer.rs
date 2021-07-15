/*!
 *  VGA text mode printing interface.
 * 
 *  Defines macros to print data using memory I/O to VGA buffer.
 */

use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

/** 
 *  Formats arguments and prints string to VGA buffer.
 */
#[macro_export]  // makes macro available to whole crate in its root
macro_rules! print {
    ($($arg:tt)*) => (
        $crate::vga_buffer::_print(format_args!($($arg)*))
    );
}

/**
 *  Formats arguments and prints string,
 *  appending a newline, to VGA buffer.
 * 
 *  If no args are given, just print a newline.
 */
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (
        $crate::print!("{}\n", format_args!($($arg)*))
    );
}

lazy_static! {  // delegates initialization to runtime and thus avoid errors
    /**
     *  Global [`Writer`] to be used as an interface.
     * 
     *  Spinlock (non-threading) [`Mutex`] guarantees
     *  synchronized safe mutability.
     */
    static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(
        Writer {
            top_row_position: BUFFER_HEIGHT - 1,
            column_position: 0,
            color_code: ColorCode::new(
                Color::Yellow, Color::Black, false
            ),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    );
}

/*---------------------------------------------------------------------------*/

/**
 *  Available colors for VGA text mode.
 * 
 *  (8 to 15 are lighter variants sometimes exclusive to foreground)
 */
#[allow(dead_code)]  // disable warnings for unused variants
#[repr(u8)]          // store each variant as `u8`
enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Magenta     = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

/** 
 *  VGA text mode color code byte.
 * 
 *  Consists of 3 bit parts:
 * 
 *  | Bit position(s) | Attribute        |
 *  |-----------------|------------------|
 *  | 0 to 3          | foreground color |
 *  | 4 to 6          | background color |
 *  | 7               | blink effect     |
 */
#[repr(transparent)]    // ensure same data layout as `u8`
#[derive(Clone, Copy)]  // allow ColorCode variables to be copied
struct ColorCode(u8);

impl ColorCode {
    /**
     *  Creates a new ColorCode by setting the respective bits in byte.
     *  
     *  If `bg >= 8`, ignore the `blink` bit instead.
     */
    const fn new(fg: Color, bg: Color, blink: bool) -> ColorCode {
        let _bg = bg as u8;
        let mut byte = (_bg << 4) + (fg as u8);
        if (_bg as u8) < 8 {
            byte += (blink as u8) << 7;
        }
        ColorCode(byte)
    }
}

/**
 *  Two-byte (sequential) structure
 *  representing a char in VGA text mode.
 */
#[repr(C)]              // guarantee correct field ordering
#[derive(Clone, Copy)]  // Copy is needed for the Volatile type
struct ScreenChar {
    /// Character's ASCII value.
    ascii_character: u8,

    /// Color code byte.
    color_code: ColorCode,
}

/// Buffer default screen height.
const BUFFER_HEIGHT: usize = 25;

/// Buffer default screen width.
const BUFFER_WIDTH: usize = 80;

/** 
 *  Matrix buffer. Points to VGA text mode's memory I/O address.
 */
#[repr(transparent)]  // ensure same memory layout as of a single field
struct Buffer {
    /// The underlying byte matrix.
    chars: [[Volatile<ScreenChar>;
            BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/**
 *  Handles writing of (colored) bytes and strings to VGA text buffer.
 */
pub struct Writer {
    /// Vertical index of topmost row.
    top_row_position: usize,

    /// Horizontal index on buffer.
    column_position: usize,

    /// Color code of bytes to be written.
    color_code: ColorCode,

    /// Pointer to memory text buffer address.
    buffer: &'static mut Buffer,
}

impl Writer {
    /** 
     *  Writes a single byte to buffer.
     */
    pub fn write_byte(&mut self, byte: u8) {
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
                // Write (two-byte) char to memory I/O buffer
                // (`write` must be used as it is of the Volatile type)
                self.buffer.chars[row][col].write(screen_char);

                // Increment column index
                self.column_position += 1;
            }
        }
    }

    /**
     *  Writes given string to buffer, byte by byte.
     */
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Char in printable range or '\n'
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Not part of printable ASCII range, so print a ■
                _ => self.write_byte(0xfe),
            }
        }
    }

    /**
     *  Iterates buffer matrix and moves each row content
     *  to the row immediately above (row 0 is just deleted instead). 
     *
     *  Thereafter a new blank line is added at the bottom.
     * */
    fn new_line(&mut self) {
        // Blank/null char
        const NULL_CHAR: ScreenChar = ScreenChar {
            ascii_character: 0,
            color_code: ColorCode(0),
        };
        // Start from topmost _written_ row (to avoid copying blank content)
        for row in self.top_row_position..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // Read Volatile ScreenChar from current position
                let screen_char = self.buffer.chars[row][col].read();
                if row > 0 {  // ignore row 0
                    // Write char to adjacent spot in previous row
                    self.buffer.chars[row - 1][col].write(screen_char);
                }
                // Delete char from current row
                self.buffer.chars[row][col].write(NULL_CHAR);
            }
        }
        // Update topmost row index and do a "carriage return" back to col 0
        if self.top_row_position > 0 {
            self.top_row_position -= 1;
        }
        self.column_position = 0;
    }
}

/**
 *  Implements `fmt::Write` trait to [`Writer`],
 *  enabling use of `write!` macro.
 */
impl fmt::Write for Writer {
    /** 
     *  Just wraps [`Writer::write_string`] call, returning successfully.
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/**
 *  Locks writer and writes formatted arguments to VGA buffer.
 */
#[doc(hidden)]  // Hide function from the docs, regardless of being public
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/*---------------------------------------------------------------------------*/

/**
 *  Simply tests if `println!` works without panicking.
 */
#[test_case]
fn test_println_simple() {
    println!("Hello world!");
}

/** 
 *  Tests printing of a few lines on screen
 *  (and shifting them off display).
 */
#[test_case]
fn test_println_many() {
    for count in 0..200 {
        println!("This is line {}", count);
    }
}

/**
 *  Ensures printed chars from string `s` are the same
 *  when reading them after operation, from second-to-last row.
 */
#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (j, c1) in s.chars().enumerate() {
        let screen_char = WRITER.lock()
                .buffer.chars[BUFFER_HEIGHT - 2][j].read();
        let c2 = char::from(screen_char.ascii_character);
        assert_eq!(c1, c2);
    }
}

/**
 *  Tests if all printable chars are correctly printed in sequence.
 * 
 *  Also tests line wrapping when reaching `BUFFER_WIDTH.
 */
#[test_case]
fn test_print_all() {
    for value in 0x20..=0x7e {
        // Write char to VGA buffer
        let c = char::from(value);
        print!("{}", c);
    }
    for value in 0x20..=0x7e {
        // Check if each char is in its correct
        let c1 = char::from(value);
        let idx = (value - 0x20) as usize;
        let mut i = idx / BUFFER_WIDTH;
        i = BUFFER_HEIGHT - 2 + i;
        let j = idx % BUFFER_WIDTH;
        let screen_char = WRITER.lock()
                .buffer.chars[i][j].read();
        let c2 = char::from(screen_char.ascii_character);
        assert_eq!(c1, c2);
    }
}
