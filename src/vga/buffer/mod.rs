
mod color;
mod color_code;
mod screen_char;

pub use color::Color;
pub use color_code::ColorCode;
pub use screen_char::ScreenChar;

use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

/// <h1>Buffer</h1>
/// Содержит упорядочнные символы. При этом имеет фиксированное количество столбцов и строк. 
#[repr(transparent)]
pub struct Buffer(pub [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]);
