use prettyt::style::{Color, Style};

fn main() {
    let style = Style::new().fg(Color::WHITE).bg(Color::BRIGHT_RED);

    println!("{}", style.apply("HELLO WORLD"));

    println!("{}", style.apply(3490239));
}
