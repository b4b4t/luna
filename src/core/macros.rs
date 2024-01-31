#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {
        println!("{}", ansi_term::Colour::Red.paint(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println_warning {
    ($($arg:tt)*) => {
        println!("{}", ansi_term::Colour::Yellow.paint(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println_info {
    ($($arg:tt)*) => {
        println!("{}", ansi_term::Colour::Blue.paint(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {
        println!("{}", ansi_term::Colour::Green.paint(format!($($arg)*)))
    };
}
