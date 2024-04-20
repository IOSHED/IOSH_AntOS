use super::ColorCode;

/// <h1>ScreenChar</h1>
/// Содержит в себе байтовое представление ASCII символа и цветовой код сивола.
/// Байты структуры располагаются в соответсвии с расположением полей структуры.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, color_code: Option<ColorCode>) -> Self {
        let color_code = color_code.unwrap_or_else(|| ColorCode::default());
        Self {
            ascii_character,
            color_code,
        }
    }
}