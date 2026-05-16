#[macro_export]
macro_rules! make_style {
    ($( $attr:ident $(($val:expr))? ),* $(,)?) => {
        {
            let s = $crate::style::Style::new();
            $(
                let s = make_style!(@impl s, $attr $(($val))?);
            )*
            s
        }
    };

    (@impl $s:expr, fg($val:expr)) => { $s.fg($val) };
    (@impl $s:expr, bg($val:expr)) => { $s.bg($val) };
    (@impl $s:expr, bold) => { $s.bold() };
    (@impl $s:expr, italic) => { $s.italic() };
    (@impl $s:expr, underline) => { $s.underline() };
    (@impl $s:expr, strikethrough) => { $s.strikethrough() };
    (@impl $s:expr, dim) => { $s.dim() };
    (@impl $s:expr, invert) => { $s.invert() };
}

#[macro_export]
macro_rules! println_styled {
    ($style:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        println!("{}", $style.apply(format!($fmt $(, $arg)*)));
    };
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, Style};

    #[test]
    fn test_make_style_empty() {
        // Ensuring an empty macro invocation yields a default Style object
        let s = make_style!();
        assert_eq!(s, Style::new());
    }

    #[test]
    fn test_make_style_single_attributes() {
        // Test simple standalone attributes
        assert!(
            make_style!(bold)
                .apply_inner("test", false)
                .contains("\x1b[1m")
        );
        assert!(
            make_style!(italic)
                .apply_inner("test", false)
                .contains("\x1b[3m")
        );
        assert!(
            make_style!(underline)
                .apply_inner("test", false)
                .contains("\x1b[4m")
        );
    }

    #[test]
    fn test_make_style_colors() {
        // Test foreground and background assignment expansions
        let s = make_style!(fg(Color::RED), bg(Color::BLUE));

        let formatted = s.apply_inner("hello", false);
        assert!(formatted.contains("hello"));
    }

    #[test]
    fn test_make_style_chained_and_trailing_comma() {
        // Verifying multiple chained attributes and handling of trailing commas
        let s = make_style!(fg(Color::GREEN), bold, italic, underline,);

        let formatted = s.apply_inner("hello", false);
        assert!(formatted.contains("\x1b[1m")); // bold
        assert!(formatted.contains("\x1b[3m")); // italic
        assert!(formatted.contains("\x1b[4m")); // underline
    }

    #[test]
    fn test_println_styled_expansion() {
        // Since println_styled! calls println!, we can verify it compiles
        // cleanly and formats correctly using a valid style object.
        let sample_style = Style::new().fg(Color::CYAN).bold();

        // This confirms the format string syntax arguments expansion compiles without issues
        println_styled!(
            sample_style,
            "Testing macro output: {} + {} = {}",
            "foo",
            "bar",
            42
        );
        println_styled!(
            sample_style,
            "Testing single string value with trailing comma",
        );
    }
}
