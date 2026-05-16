use prettyt::style::{Color, Style};

fn main() {
    // --- Basic ANSI 16 colors ---
    let error = Style::new().fg(Color::BRIGHT_RED).bold();
    let success = Style::new().fg(Color::BRIGHT_GREEN).bold();
    let warning = Style::new().fg(Color::YELLOW);
    let muted = Style::new().fg(Color::BRIGHT_BLACK); // gray

    println!("{} Something went wrong", error.apply("[ERROR]"));
    println!("{} Build passed in 0.43s", success.apply("[OK]"));
    println!("{} Deprecated API used", warning.apply("[WARN]"));
    println!("{}", muted.apply("-- details omitted --"));

    println!();

    // --- 256-color palette ---
    let header = Style::new()
        .fg(Color::Ansi256(220)) // gold
        .bg(Color::Ansi256(235)) // dark gray background
        .bold();

    println!("{}", header.apply(" prettyt color showcase "));

    println!();

    // --- True RGB colors ---
    let coral = Style::new().fg(Color::Rgb(255, 100, 80));
    let teal = Style::new().fg(Color::Rgb(64, 200, 180));
    let violet = Style::new().fg(Color::Rgb(180, 80, 255));

    println!(
        "{} {} {}",
        coral.apply("coral"),
        teal.apply("teal"),
        violet.apply("violet"),
    );

    println!();

    // --- Composing styles for a log line ---
    let timestamp = Style::new().fg(Color::BRIGHT_BLACK);
    let level_ok = Style::new().fg(Color::GREEN);
    let message = Style::new().fg(Color::WHITE);

    let ts = timestamp.apply("[12:34:56]");
    let lvl = level_ok.apply("INFO ");
    let msg = message.apply("Server started on port 8080");

    println!("{} {} {}", ts, lvl, msg);

    // --- Numeric values work too ---
    let highlight = Style::new().fg(Color::CYAN).bold();
    println!("Result:   {}", highlight.apply(42));
    println!("Uptime:   {}", highlight.apply(99.97));
}
