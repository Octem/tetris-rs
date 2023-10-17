use crossterm::style::Color;

pub fn get_color(index: u8) -> Color {
    match index {
        1 => Color::Red,
        2 => Color::Blue,
        3 => Color::Rgb { r: 175, g: 95, b: 0 },
        4 => Color::Yellow,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Green,
        _ => Color::White, 
    }
}