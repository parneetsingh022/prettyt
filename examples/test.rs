use prettyt::{Color, make_style, sprintln};

fn main() {
    sprintln!(make_style!(fg(Color::Red), strikethrough), "HELLO WORLD");
}
