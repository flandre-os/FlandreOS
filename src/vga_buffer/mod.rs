use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::vga_buffer::color::{Color, ColorCode};
use crate::vga_buffer::writer::Writer;

pub mod buffer;
pub mod color;
pub mod writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
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

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    let color = writer.color();
    let error_color = ColorCode::new(Color::Red, Color::Black);
    writer.set_color(error_color);
    writer.write_fmt(args).unwrap();
    writer.set_color(color);
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
            let screen_char = WRITER.lock().read_buffer(BUFFER_HEIGHT - 2, i);
            assert_eq!(char::from(screen_char.ascii_character), c);
        }

        serial_println!("[ok]");
    }
}
