pub enum Colour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
    Rgb(u8, u8, u8),
}

impl Colour {
    pub fn colour(&self) -> u32 {
        match self {
            Colour::Black => 0xff000000,
            Colour::Blue => 0xff800000,
            Colour::Green => 0xff008000,
            Colour::Cyan => 0xff808000,
            Colour::Red => 0xff000080,
            Colour::Magenta => 0xff800080,
            Colour::Brown => 0xff008080,
            Colour::LightGray => 0xff808080,
            Colour::DarkGray => 0xff404040,
            Colour::LightBlue => 0xffff0000,
            Colour::LightGreen => 0xff00ff00,
            Colour::LightCyan => 0xffffff00,
            Colour::LightRed => 0xff0000ff,
            Colour::LightMagenta => 0xffff00ff,
            Colour::Yellow => 0xff00ffff,
            Colour::White => 0xffffffff,
            Colour::Rgb(r, g, b) => {
                let r = *r as u32;
                let g = *g as u32;
                let b = *b as u32;
                0xff000000 | (r << 16) | (g << 8) | b
            }
        }
    }
}

impl From<Colour> for u32 {
    fn from(colour: Colour) -> Self {
        colour.colour()
    }
}
