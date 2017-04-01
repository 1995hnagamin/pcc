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

pub enum Command {
    Push(i32),
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InNumber,
    InChar,
    OutNumber,
    OutChar,
}
