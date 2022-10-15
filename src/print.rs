/// Provides printing macros



use core::fmt;

use crate::console;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // todo: handle result.
    console::console().write_fmt(args);
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        _print(format_args_nl!($($arg)*));
    })
}