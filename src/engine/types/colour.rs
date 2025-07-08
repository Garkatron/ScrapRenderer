#![allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum COLOUR {
    BLACK   = 0x000000,
    WHITE   = 0xFFFFFF,
    RED     = 0xFF0000,
    GREEN   = 0x00FF00,
    BLUE    = 0x0000FF,
    YELLOW  = 0xFFFF00,
    CYAN    = 0x00FFFF,
    MAGENTA = 0xFF00FF,
    GRAY    = 0x808080,
    ORANGE  = 0xFFA500,
    PINK    = 0xFFC0CB,
    BROWN   = 0x8B4513,
}

impl COLOUR {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

pub enum PALETTE_INKPINK {
    WHITE = 0xffffff,
    PINK0 = 0xfe6c90,
    PINK1 = 0xd03791,
    PINK2 = 0x87286a,
    PINK3 = 0x452459,
    PINK4 = 0x260d34,
}

impl PALETTE_INKPINK {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}