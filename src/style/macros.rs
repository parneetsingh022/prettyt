#[macro_export]
macro_rules! make_style {
    ($( $attr:ident $(($val:expr))? ),* $(,)?) => {
        {
            let s = $crate::style::Style::new();
            $(
                let s = $crate::make_style!(@impl s, $attr $(($val))?);
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
macro_rules! sprintln {
    ($style:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        println!("{}", $style.apply(&format_args!($fmt $(, $($arg)*)?)));
    };
}

#[cfg(test)]
mod tests {
    use crate::style::{Color, Style};
    use crate::terminal::{ColorLevel, TerminalApp};
    use crate::test_utils::MockTerminalGuard;

    #[test]
    fn test_make_style_empty() {
        // Ensuring an empty macro invocation yields a default Style object
        let s = make_style!();
        assert_eq!(s, Style::new());
    }

    #[test]
    fn test_make_style_single_attributes() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);

        assert!(format!("{}", make_style!(bold).apply("test")).contains("\x1b[1m"));
        assert!(format!("{}", make_style!(italic).apply("test")).contains("\x1b[3m"));
        assert!(format!("{}", make_style!(underline).apply("test")).contains("\x1b[4m"));
    }

    #[test]
    fn test_make_style_colors() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        let s = make_style!(fg(Color::Red), bg(Color::Blue));

        let formatted = format!("{}", s.apply("hello"));
        assert!(formatted.contains("hello"));
    }

    #[test]
    fn test_make_style_chained_and_trailing_comma() {
        let _guard = MockTerminalGuard::acquire(TerminalApp::Unknown, ColorLevel::TrueColor);
        let s = make_style!(fg(Color::Green), bold, italic, underline,);

        let formatted = format!("{}", s.apply("hello"));
        assert!(formatted.contains("\x1b[1m"));
        assert!(formatted.contains("\x1b[3m"));
        assert!(formatted.contains("\x1b[4m"));
    }

    #[test]
    fn test_sprintln_expansion() {
        // Since sprintln! calls println!, we can verify it compiles
        // cleanly and formats correctly using a valid style object.
        let sample_style = Style::new().fg(Color::Cyan).bold();

        // This confirms the format string syntax arguments expansion compiles without issues
        sprintln!(
            sample_style,
            "Testing macro output: {} + {} = {}",
            "foo",
            "bar",
            42
        );
        sprintln!(
            sample_style,
            "Testing single string value with trailing comma",
        );
    }

    #[test]
    fn test_sprintln_with_named_args() {
        let sample_style = crate::style::Style::new().bold();

        // This will now compile flawlessly!
        sprintln!(
            sample_style,
            "Welcome back, {user}! Dev server status: {status}",
            user = "Parneet",
            status = "Active"
        );
    }
}
