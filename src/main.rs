//! Точка входа

#![no_std]    // отключаем стандартную библиотеку
#![no_main]   // отключаем стандартную цепочку вызовов при инициализации

use core::panic::PanicInfo;


/// **panic:**
/// Кастомная функция, заменяющая функцию из std lib. Вызывается при панике кода.
///
/// **args:**
/// - `info`: `&PanicInfo` - содержит соощение об ошибке и строку кода,
///                          в которой была вызвана `panic`.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Последовательность символов для вывода на экран.
static HELLO: &[u8] = b"Hello World!";

/// **_start:**
/// Функция без искажения имени при кмпиляции. Она является токой фхода в OS.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
