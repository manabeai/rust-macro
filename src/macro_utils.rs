//! Macros for competitive programming

/// Prints values to stdout with space separation
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let stdout = std::io::stdout();
        let mut handle = std::io::BufWriter::new(stdout.lock());
        write!(handle, "{}", format_args!($($arg)*)).unwrap();
    }};
}

/// Prints values to stdout with space separation and adds newline
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let stdout = std::io::stdout();
        let mut handle = std::io::BufWriter::new(stdout.lock());
        writeln!(handle, "{}", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! printvec {
    ($vec:expr) => {{
        let joined = $vec
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", joined);
    }};
}

#[cfg(test)]
mod tests {
    // use super::*;

    use std::io::Write;

    #[test]
    fn test_print_macro() {
        let mut output = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut output);
            let _ = write!(cursor, "{}", "Hello");
            let _ = write!(cursor, "{}", " ");
            let _ = write!(cursor, "{}", "World");
        }
        assert_eq!(output, b"Hello World");
    }

    #[test]
    fn test_println_macro() {
        let mut output = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut output);
            let _ = writeln!(cursor, "{}", "Hello");
            let _ = writeln!(cursor, "{}", "World");
        }
        assert_eq!(output, b"Hello\nWorld\n");
    }
}
