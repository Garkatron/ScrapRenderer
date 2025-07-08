#![allow(non_camel_case_types)]

use crate::engine::rendering::palette::Palette;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PALETTE_DEFAULT {
    BLACK   = 0x000000,
    WHITE   = 0xFFFFFF,
    RED     = 0xFF0000,
    GREEN   = 0x00FF00,
    BLUE    = 0x0000FF,
    YELLOW  = 0xFFFF00,
    ORANGE  = 0xFFA500,
}

impl PALETTE_DEFAULT {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

pub struct PaletteDefault;

impl Palette for PaletteDefault {
    fn get_shading_color(&self, dp: f32) -> u32 {
        match (dp * 3.0) as i32 {
            0 => PALETTE_DEFAULT::ORANGE.to_u32(),
            1 => PALETTE_DEFAULT::YELLOW.to_u32(),
            2 => PALETTE_DEFAULT::WHITE.to_u32(),
            _ => PALETTE_DEFAULT::BLACK.to_u32(),
        }
    }
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PALETTE_PINK {
    WHITE = 0xFFFFFF,
    PINK0 = 0xFE6C90,
    PINK1 = 0xD03791,
    PINK2 = 0x87286A,
    PINK3 = 0x452459,
    PINK4 = 0x260D34,
}

impl PALETTE_PINK {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

pub struct PalettePink;

impl Palette for PalettePink {
    fn get_shading_color(&self, dp: f32) -> u32 {
        match (dp * 4.0) as i32 {
            0 => PALETTE_PINK::PINK4.to_u32(),
            1 => PALETTE_PINK::PINK3.to_u32(),
            2 => PALETTE_PINK::PINK2.to_u32(),
            3 => PALETTE_PINK::PINK1.to_u32(),
            4 => PALETTE_PINK::PINK0.to_u32(),
            _ => PALETTE_DEFAULT::BLACK.to_u32(),
        }
    }
}
