
mod struct_printer;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use struct_printer::Printer;
use crate::vga::buffer::Color;

lazy_static! {
    static ref PRINTER: Mutex<Printer> = Mutex::new(Printer::default());
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    PRINTER.lock().write_fmt(args).unwrap();
}

/// <h1>set_color_for_printer</h1>
/// Устанавливает цвет для печатаемого текста.
///
/// **Args:**
/// - foreground (Color) - цвет для шрифта.
/// - background (Color) - цвет для фона.
///
/// **Example use:**
/// ```rust
/// use crate::vga::printer::set_color_for_printer;
/// use crate::vga::Color;
///
/// println("Hello, world!");    // печатает со стандартными цветами.
/// set_color_for_printer(Color::Red, Color::Yellow);
/// println("Hello, red color!");   // печатает с красным шрифтом и жёлтым фоном.
/// ```
pub fn set_color_for_printer(foreground: Color, background: Color) {
    PRINTER.lock().set_color_code(foreground, background);
}

/// <h1>set_color_for_printer</h1>
/// Устанавливает стандартный цвет для печатаемого текста.
///
/// **Example use:**
/// ```rust
/// use crate::vga::printer::{set_color_for_printer, reset_color_for_printer};
/// use crate::vga::Color;
///
/// set_color_for_printer(Color::Red, Color::Yellow);
/// println("Hello, red color!");   // печатает с красным шрифтом и жёлтым фоном.
///
/// reset_color_for_printer();
/// println("Hello, default!");     // печатает со стандартными цветами.
///
/// ```
pub fn reset_color_for_printer() {
    PRINTER.lock().reset_color_code();
}

/// <h1>print!</h1>
/// Печатает текст в одной строке.
///
/// **Example use:**
/// ```rust
///
/// print!("Hello!");     // печатает `Hello!`.
/// print!("Wörld!");     // печатает `W■■rld!`, так как ö не ASCII символ.
/// print!("The numbers are {} and {}", 42, 1.0/3.0)     // печатает `The numbers are 42 and 0.33333`
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::printer::_print(format_args!($($arg)*)));
}

/// <h1>println!</h1>
/// Печатает текст и переходит на новую линию.
///
/// **Example use:**
/// ```rust
///
/// println!("Hello!");     // печатает `Hello!\n`.
/// println!("Wörld!");     // печатает `W■■rld!\n`, так как ö не ASCII символ.
/// println!("The numbers are {} and {}", 42, 1.0/3.0)     // печатает `The numbers are 42 and 0.33333\n`.
/// ```
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
