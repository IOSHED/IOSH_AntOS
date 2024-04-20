use super::Color;


/// <h1>ColorCode</h1>
/// Содержит байт со значением увета текста и цвета фона
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// <h1>new</h1>
    /// Создаёт структуру, содержащую байт, прежде рассчитаный в этой функции, со значением цвета текста и цвета фона.
    pub fn new(foreground: Option<Color>, background: Option<Color>) -> Self {
        let foreground = foreground.unwrap_or_else(|| Color::Cyan);
        let background = background.unwrap_or_else(|| Color::Black);

        Self((background as u8) << 4 | (foreground as u8))
    }
}

impl Default for ColorCode {
    /// <h1>default</h1>
    /// Создает стандарный цветовой код для символа.
    fn default() -> Self {
        Self::new(None, None)
    }
}