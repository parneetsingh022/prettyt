use prettyt::style::{Color, Style};

fn main() {
    let style = Style::new()
        .fg(Color::Rgb(255, 255, 115))
        .bg(Color::BRIGHT_RED);

    println!("{}", style.apply("HELLO WORLD"));

    println!("{}", style.apply(3490239));
}
