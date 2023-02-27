pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}


impl Color {
    fn ansi_code(&self) -> &str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        }
    }
}

#[macro_export]
macro_rules! println_colored {
    ($color:expr, $fmt:expr) => ({
        println!("{}{}{}", $color.ansi_code(), $fmt, Color::Reset.ansi_code());
    });
    ($color:expr, $fmt:expr, $($arg:tt)*) => ({
        println!("{}{}{}{}", $color.ansi_code(), format_args!($fmt, $($arg)*), Color::Reset.ansi_code());
    });
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_red() {
        assert_eq!(
            format!("{}{}{}", Color::Red.ansi_code(), "hello", Color::Reset.ansi_code()),
            "\x1b[31mhello\x1b[0m"
        );
    }
    #[test]
    fn test_green() {
        assert_eq!(
            format!("{}{}{}", Color::Green.ansi_code(), "world", Color::Reset.ansi_code()),
            "\x1b[32mworld\x1b[0m"
        );
    }
    #[test]
    fn test_yellow() {
        assert_eq!(
            format!("{}{}{}", Color::Yellow.ansi_code(), "foo", Color::Reset.ansi_code()),
            "\x1b[33mfoo\x1b[0m"
        );
    }
    #[test]
    fn test_blue() {
        assert_eq!(
            format!("{}{}{}", Color::Blue.ansi_code(), "bar", Color::Reset.ansi_code()),
            "\x1b[34mbar\x1b[0m"
        );
    }
    #[test]
    fn test_magenta() {
        assert_eq!(
            format!("{}{}{}", Color::Magenta.ansi_code(), "baz", Color::Reset.ansi_code()),
            "\x1b[35mbaz\x1b[0m"
        );
    }

    #[test]
    fn test_cyan() {
        assert_eq!(
            format!("{}{}{}", Color::Cyan.ansi_code(), "qux", Color::Reset.ansi_code()),
            "\x1b[36mqux\x1b[0m"
        );
    }
    #[test]
    fn test_black() {
        assert_eq!(
            format!("{}{}{}", Color::Black.ansi_code(), "hello", Color::Reset.ansi_code()),
            "\x1b[30mhello\x1b[0m"
        );
    }

    #[test]
    fn test_white() {
        assert_eq!(
            format!("{}{}{}", Color::White.ansi_code(), "world", Color::Reset.ansi_code()),
            "\x1b[37mworld\x1b[0m"
        );
    }

}




