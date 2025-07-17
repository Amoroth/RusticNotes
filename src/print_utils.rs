pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }
}

pub fn colorize(color: Color, input: &String) -> String {
    String::from(format!("\x1b[38;2;{};{};{}m", color.red, color.green, color.blue)) + input.as_str() + "\x1b[0m"
}

pub fn bg_colorize(color: Color, input: &String) -> String {
    String::from(format!("\x1b[48;2;{};{};{}m", color.red, color.green, color.blue)) + input.as_str() + "\x1b[0m"
}
