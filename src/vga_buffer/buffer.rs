use volatile::Volatile;

use crate::vga_buffer::color::ColorCode;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub const MAX_ROW: usize = BUFFER_HEIGHT - 1;
pub const MAX_COLUMN: usize = BUFFER_WIDTH - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn write(&mut self, row: usize, column: usize, character: ScreenChar) {
        self.chars[row][column].write(character);
    }

    pub fn read(&self, row: usize, column: usize) -> ScreenChar {
        self.chars[row][column].read()
    }
}
