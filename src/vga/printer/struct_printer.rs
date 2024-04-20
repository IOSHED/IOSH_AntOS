use core::fmt;
use crate::vga::buffer::Color;

use super::super::buffer::{Buffer, ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

/// <h1>Printer</h1>
/// Печатает символы и строки в буфер.
pub struct Printer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


impl Printer {
    pub fn new(column_position: usize, color_code: ColorCode, buffer: &'static mut Buffer) -> Self {
        Self {
            column_position,
            color_code,
            buffer,
        }
    }

    /// <h1>set_color_code</h1>
    /// Устанавливает новый цвет для последующего напечатанного текста.
    pub fn set_color_code(&mut self, foreground: Color, background: Color) {
        let new_color_code = ColorCode::new(Some(foreground), Some(background));
        self.color_code = new_color_code;
    }

    /// <h1>reset_color_code</h1>
    /// Устанавливает стандартный цвет для последующего напечатанного текста.
    pub fn reset_color_code(&mut self) {
        self.color_code = ColorCode::default();
    }
    
    /// <h1>write_string</h1>
    /// Получает строку и печатает её в буффер. Символ '\n' переновит на следующую строку.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // байты ASCII или байт '\n'
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }

    /// <h1>write_byte</h1>
    /// Получает байт и помещает его в буффер. Байт '\n' переновит на следующую строку.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => self.new_byte(byte),
        }
    }

    /// <h1>new_line</h1>
    /// Переходит на новую строку в буффре.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.0[row][col].read();
                self.buffer.0[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// <h1>clear_row</h1>
    /// Очищает строку, вставляя b' '.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', Some(self.color_code));
        for col in 0..BUFFER_WIDTH {
            self.buffer.0[row][col].write(blank);
        }
    }

    /// <h1>new_byte</h1>
    /// Помещает байт в буффер после предыдущего байта.
    fn new_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = Some(self.color_code);
        let screen_char = ScreenChar::new(byte, color_code);
        self.buffer.0[row][col].write(screen_char);
        self.column_position += 1;
    }
}

impl Default for Printer {
    /// <h1>default</h1>
    /// Создаёт стандартный Printer. Его буфер указывает на байт 0xb8000,
    /// который является началом выделенного пространсва памяти VGA.
    fn default() -> Self {
        Self { column_position: 0, color_code: ColorCode::default(), buffer: unsafe { &mut *(0xb8000 as *mut Buffer) } }
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}