use prettyt::{Color, make_style};

fn main() {
    let style = make_style!(fg(Color::Red), bg(Color::White));

    println!("{}", style.apply("HELLO WORLD"));
}
