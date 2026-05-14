use prettyt::style::{Color, Style};

fn main() {
    let style = Style::new()
        .fg(Color::Rgb(58, 117, 189))
        .bg(Color::Rgb(245, 73, 39));

    println!("{}", style.apply("HELLO WORLD"));

    println!("{}", style.apply(3490239));
}
