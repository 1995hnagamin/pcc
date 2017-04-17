#[derive(Clone, Copy, PartialEq)]
pub enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Lightness {
    Light,
    Normal,
    Dark,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
    Chromatic(Hue, Lightness),
}

#[derive(Clone, Copy, PartialEq)]
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

fn integer_of_hue(h: Hue) -> usize {
    use self::Hue::*;
    match h {
        Red => 0,
        Yellow => 1,
        Green => 2,
        Cyan => 3,
        Blue => 4,
        Magenta => 5,
    }
}

fn integer_of_lightness(l: Lightness) -> usize {
    use self::Lightness::*;
    match l {
        Light => 0,
        Normal => 1,
        Dark => 2,
    }
}

const HUE_CYCLE: usize = 6;
const LIGHTNESS_CYCLE: usize = 3;

fn get_command_from_map(h: usize, l: usize, area: u32) -> Command {
    assert!(h < HUE_CYCLE);
    assert!(l < LIGHTNESS_CYCLE);
    use self::Command::*;
    let map = [[Nop, Push(area), Pop],
               [Add, Subtract, Multiply],
               [Divide, Mod, Not],
               [Greater, Pointer, Switch],
               [Duplicate, Roll, InNumber],
               [InChar, OutNumber, OutChar]];
    map[h][l]
}

fn subtract_cyclically(dst: usize, src: usize, cycle: usize) -> usize {
    assert!(dst < cycle);
    assert!(src < cycle);
    (cycle + dst - src) % cycle
}

pub fn get_command(src: Color, dst: Color) -> Command {
    use self::Color::*;
    match (src, dst) {
        (Chromatic(sh, sl), Chromatic(dh, dl)) => {
            let dh = integer_of_hue(dh);
            let sh = integer_of_hue(sh);
            let h = subtract_cyclically(dh, sh, HUE_CYCLE);

            let dl = integer_of_lightness(dl);
            let sl = integer_of_lightness(sl);
            let l = subtract_cyclically(dl, sl, LIGHTNESS_CYCLE);

            get_command_from_map(h, l, 0)
        }
        (_, _) => Command::Nop,
    }
}
