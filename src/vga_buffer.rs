/**
 *  VGA text mode module.
 *  Prints characters on screen by writting to memory I/O buffer.
 */

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]  // makes macro available to whole crate and place it on root
macro_rules! print {
    /* Format arguments and print to VGA buffer. */
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    /* Format arguments and print, appending a newline, to VGA buffer.
        If no args are given, just print a newline. */
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

lazy_static! {  // delegates initialization to runtime and thus avoid errors
    // Global writer to be used as an interface.
    // Spinlock (non-threading) Mutex enables synchronized safe mutability.
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        top_row_position: BUFFER_HEIGHT - 1,
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black, false),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/*---------------------------------------------------------------------------*/

#[allow(dead_code)]  // disable warnings for unused variants
#[repr(u8)]          // store each variant as `u8`
enum Color {
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

    const fn new(fg: Color, bg: Color, blink: bool) -> ColorCode {
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
    top_row_position: usize,      // vertical index of topmost row
    column_position: usize,       // horizontal index on buffer
    color_code: ColorCode,        // color code of the following bytes
    buffer: &'static mut Buffer,  // pointer to memory text buffer
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
                // Write (two-byte) char to memory I/O buffer
                // (`write` must be used as it is of the Volatile type)
                self.buffer.chars[row][col].write(screen_char);

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
    fn new_line(&mut self) {
        /* Iterate buffer matrix and move each row content
            to the row immediately above (row 0 is just deleted instead). 
            Process results then in a new blank line added at the bottom. */

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

impl fmt::Write for Writer {
    /* Implements Write trait to Writer to enable use of `write!` macro. */

    fn write_str(&mut self, s: &str) -> fmt::Result {
        /* Required trait method.
            Just wraps `Writer::write_string` call and returns success. */
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]  // Hide function from the docs, regardless of being public
pub fn _print(args: fmt::Arguments) {
    /* Lock writer and write formatted arguments to VGA buffer. */
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/*---------------------------------------------------------------------------*/

#[test_case]
fn test_println_simple() {
    /* Simply test if `println!` works without panicking. */
    println!("Hello world!");
}

#[test_case]
fn test_println_many() {
    /* Test printing many lines on screen
        (and shifting them off display). */
    for count in 0..200 {
        println!("This is line {}", count);
    }
}

#[test_case]
fn test_println_output() {
    /*  Ensure printed chars from string `s` are the same
        when reading them after operation, from second-to-last row. */
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (j, c1) in s.chars().enumerate() {
        let screen_char = WRITER.lock()
                .buffer.chars[BUFFER_HEIGHT - 2][j].read();
        let c2 = char::from(screen_char.ascii_character);
        assert_eq!(c1, c2);
    }
}

#[test_case]
fn test_print_all() {
    /* Test if all printable chars are correctly printed in sequence..
        Also tests line wrapping when reaching BUFFER_WIDTH. */
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
