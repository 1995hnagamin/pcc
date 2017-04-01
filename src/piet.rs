pub enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

pub enum Lightness {
    Light,
    Normal,
    Dark,
}

pub enum Color {
    White,
    Black,
    Chromatic(Hue, Lightness),
}
