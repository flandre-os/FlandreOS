use core::fmt;

use crate::vga_buffer::buffer::{
    Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, MAX_COLUMN, MAX_ROW,
};
use crate::vga_buffer::color::ColorCode;

pub struct Writer {
    row: usize,
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Default for Writer {
    fn default() -> Self {
        Writer {
            row: 0,
            column: 0,
            color: ColorCode::default(),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
}

impl Writer {
    pub fn size() -> (usize, usize) {
        (BUFFER_HEIGHT, BUFFER_WIDTH)
    }

    pub fn position(&self) -> (usize, usize) {
        (self.row(), self.column())
    }

    pub fn set_position(&mut self, row: usize, column: usize) {
        self.set_row(row);
        self.set_column(column);
    }

    pub fn color(&self) -> ColorCode {
        self.color
    }

    pub fn set_color(&mut self, value: ColorCode) {
        self.color = value;
    }

    pub fn read_buffer(&self, row: usize, column: usize) -> ScreenChar {
        self.buffer.read(row, column)
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = self.blank_character();
        for column in 0..=MAX_COLUMN {
            self.buffer.write(row, column, blank);
        }
    }

    pub fn clear_screen(&mut self) {
        let blank = self.blank_character();
        for row in 0..=MAX_ROW {
            for column in 0..=MAX_COLUMN {
                self.buffer.write(row, column, blank);
            }
        }
    }

    pub fn scroll_up(&mut self) {
        for row in 0..MAX_ROW {
            for column in 0..=MAX_COLUMN {
                let character = self.buffer.read(row + 1, column);
                self.buffer.write(row, column, character);
            }
        }
        self.clear_row(MAX_ROW);
    }

    pub fn scroll_down(&mut self) {
        for row in (1..=MAX_ROW).rev() {
            for column in 0..=MAX_COLUMN {
                let character = self.buffer.read(row - 1, column);
                self.buffer.write(row, column, character);
            }
        }
        self.clear_row(0);
    }
}

impl Writer {
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.set_column(0),
            0x20..=0x7e => {
                let character = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color(),
                };
                self.buffer.write(self.row(), self.column(), character);

                let current_column = self.column();
                if current_column >= MAX_COLUMN {
                    self.new_line();
                } else {
                    self.set_column(current_column + 1);
                }
            }
            _ => {
                self.write_byte(0xfe);
            }
        }
    }

    fn new_line(&mut self) {
        let current_row = self.row();
        if current_row >= MAX_ROW {
            self.scroll_up();
        } else {
            self.set_row(current_row + 1);
        }

        self.set_column(0);
    }

    fn blank_character(&self) -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: self.color(),
        }
    }

    fn row(&self) -> usize {
        self.row
    }

    fn set_row(&mut self, value: usize) {
        debug_assert!(value < BUFFER_HEIGHT);
        self.row = value;
    }

    fn column(&self) -> usize {
        self.column
    }

    fn set_column(&mut self, value: usize) {
        debug_assert!(value < BUFFER_WIDTH);
        self.column = value;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
