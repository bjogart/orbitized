use mottle::style::Color;

pub(crate) struct Palette {
    pub bg: Color,
    pub em_bg: Color,
    pub sub_fg: Color,
    pub fg: Color,
    pub em_fg: Color,
    pub yellow: Color,
    pub orange: Color,
    pub red: Color,
    pub magenta: Color,
    pub violet: Color,
    pub blue: Color,
    pub cyan: Color,
    pub green: Color,
}

impl Palette {
    const BASE03: Color = hex_color(0x002b36);
    const BASE02: Color = hex_color(0x073642);
    const BASE01: Color = hex_color(0x586e75);
    const BASE00: Color = hex_color(0x657b83);
    const BASE0: Color = hex_color(0x839496);
    const BASE1: Color = hex_color(0x93a1a1);
    const BASE2: Color = hex_color(0xeee8d5);
    const BASE3: Color = hex_color(0xfdf6e3);
    const YELLOW: Color = hex_color(0xb58900);
    const ORANGE: Color = hex_color(0xcb4b16);
    const RED: Color = hex_color(0xdc322f);
    const MAGENTA: Color = hex_color(0xd33682);
    const VIOLET: Color = hex_color(0x6c71c4);
    const BLUE: Color = hex_color(0x268bd2);
    const CYAN: Color = hex_color(0x2aa198);
    const GREEN: Color = hex_color(0x859900);

    pub fn luna() -> Self {
        Self {
            bg: Self::BASE03,
            em_bg: Self::BASE02,
            sub_fg: Self::BASE01,
            fg: Self::BASE0,
            em_fg: Self::BASE1,
            yellow: Self::YELLOW,
            orange: Self::ORANGE,
            red: Self::RED,
            magenta: Self::MAGENTA,
            violet: Self::VIOLET,
            blue: Self::BLUE,
            cyan: Self::CYAN,
            green: Self::GREEN,
        }
    }

    pub fn sol() -> Self {
        Self {
            bg: Self::BASE3,
            em_bg: Self::BASE2,
            sub_fg: Self::BASE1,
            fg: Self::BASE00,
            em_fg: Self::BASE01,
            yellow: Self::YELLOW,
            orange: Self::ORANGE,
            red: Self::RED,
            magenta: Self::MAGENTA,
            violet: Self::VIOLET,
            blue: Self::BLUE,
            cyan: Self::CYAN,
            green: Self::GREEN,
        }
    }
}

pub const fn hex_color(hex: u32) -> Color {
    Color { hex, alpha: None }
}

pub const fn with_alpha(c: Color, a: u8) -> Color {
    Color { hex: c.hex, alpha: Some(a) }
}
