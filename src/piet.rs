#[derive(Clone, Copy)]
pub enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

#[derive(Clone, Copy)]
pub enum Lightness {
    Light,
    Normal,
    Dark,
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
    Chromatic(Hue, Lightness),
}

#[derive(Clone, Copy)]
pub enum Command {
    Nop,
    Push(u32),
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
