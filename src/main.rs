//! Точка входа

#![no_std]    // отключаем стандартную библиотеку
#![no_main]   // отключаем стандартную цепочку вызовов при инициализации

use core::panic::PanicInfo;

use crate::vga::Color;
use crate::vga::printer::set_color_for_printer;

mod vga;

/// <h1>panic</h1>
/// Кастомная функция, заменяющая функцию из std lib. Вызывается при панике кода.
///
/// **args:**
/// - `info`: `&PanicInfo` - содержит соощение об ошибке и строку кода,
///                          в которой была вызвана `panic`.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color_for_printer(Color::Red, Color::Yellow);
    println!("{}", info);
    loop {}
}

/// <h1>_start</h1>
/// Функция без искажения имени при кмпиляции. Она является токой фхода в OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi, my bro!");
    println!("Delay 5s.");
    loop {}
}
