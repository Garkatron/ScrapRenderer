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
