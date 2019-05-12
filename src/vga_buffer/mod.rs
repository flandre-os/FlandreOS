use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::vga_buffer::buffer::Buffer;
use crate::vga_buffer::color::{Color, ColorCode};
use crate::vga_buffer::writer::Writer;

mod buffer;
mod color;
mod writer;

pub const DEFAULT_FOREGROUND: Color = Color::Green;
pub const DEFAULT_BACKGROUND: Color = Color::Black;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
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

mod test {
    #[cfg(test)]
    use crate::vga_buffer::buffer::BUFFER_HEIGHT;
    #[cfg(test)]
    use crate::vga_buffer::WRITER;
    #[cfg(test)]
    use crate::{serial_print, serial_println};

    #[test_case]
    fn test_println_simple() {
        serial_print!("test_println_simple... ");
        println!("test_println_simple output");
        serial_println!("[ok]");
    }

    #[test_case]
    fn test_println_many() {
        serial_print!("test_println_many... ");
        for _ in 0..200 {
            println!("test_println_many output");
        }
        serial_println!("[ok]");
    }

    #[test_case]
    fn test_println_output() {
        serial_print!("test_println_output... ");

        let s = "Some test string that fits on a single line";
        println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }

        serial_println!("[ok]");
    }
}
