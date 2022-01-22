#[macro_export]
macro_rules! print_flush {
    ($($arg:tt)*) => {
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    };
}

#[macro_export]
macro_rules! clear_screen {
    () => {
        print_flush!("\x1B[2J");
    };
}

#[macro_export]
macro_rules! set_cursor_position {
    ($x:expr, $y:expr) => {
        print_flush!("\x1B[{};{}H", $y, $x);
    };
}

#[macro_export]
macro_rules! stdin_read_line {
    ($x:expr) => {
        $x.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line($x).unwrap();
    };
}

pub(crate) use print_flush;
pub(crate) use clear_screen;
pub(crate) use set_cursor_position;
pub(crate) use stdin_read_line;
