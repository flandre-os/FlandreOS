use crate::vga_buffer::buffer::Buffer;
use crate::vga_buffer::color::{Color, ColorCode};
use crate::vga_buffer::writer::Writer;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

mod buffer;
mod color;
mod writer;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
