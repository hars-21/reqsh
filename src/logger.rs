use std::{
    fmt::Arguments,
    io::{self, IsTerminal},
};

/// Prints a formatted error message to std out if it's a terminal,
/// or stderr if it is being redirected (e.g., piped to a file).
pub fn print_error(args: Arguments) {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        println!("{}", args);
    } else {
        eprintln!("{}", args);
    }
}

/// Helper macro to mimic standard print macro syntax
/// Prints a formatted error message to stdout if it's a terminal,
/// or stderr if it is being redirected (e.g., piped to a file).
#[macro_export]
macro_rules! logerror {
    ($($arg:tt)*) => {
        $crate::logger::print_error(format_args!($($arg)*))
    };
}
