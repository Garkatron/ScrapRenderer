use core::fmt;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use image::{Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    PNG,
    JPG,
    BMP,
}

#[derive(Debug)]
pub struct ParseImageFormatError;

impl fmt::Display for ParseImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid image format")
    }
}

impl Error for ParseImageFormatError {}

impl FromStr for ImageFormat {
    type Err = ParseImageFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::PNG),
            "jpg" | "jpeg" => Ok(ImageFormat::JPG),
            "bmp" => Ok(ImageFormat::BMP),
            _ => Err(ParseImageFormatError),
        }
    }
}
#[derive(Debug)]
pub struct Texture {
    pub path: PathBuf,
    pub format: ImageFormat,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn load(path: &str) -> Result<Texture, Box<dyn Error>> {
        // Load the image using the image crate
        let img = image::open(path)?;
        
        // Convert to RGBA format for consistent pixel data
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let data = rgba.into_raw();

        let binding = PathBuf::from(path);
        let extension = binding
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| ParseImageFormatError)?;

        let format = ImageFormat::from_str(extension)?;

        Ok(Texture {
            path: PathBuf::from(path),
            format,
            data,
            width,
            height,
        })
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba<u8>> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize * 4; // RGBA has 4 bytes per pixel
        if index + 3 < self.data.len() {
            Some(Rgba([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ]))
        } else {
            None
        }
    }

    pub fn get_pixel_as_u32(&self, x: u32, y: u32, alpha: bool) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }
    
        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 >= self.data.len() {
            return None;
        }
    
        let r = self.data[index] as u32;
        let g = self.data[index + 1] as u32;
        let b = self.data[index + 2] as u32;
        let a = self.data[index + 3] as u32;
    
        let color_u32 = if alpha {
            // RGBA: red in highest byte
            (r << 24) | (g << 16) | (b << 8) | a
        } else {
            // RGB only, alpha byte is 0
            (r << 16) | (g << 8) | b
        };
    
        Some(color_u32)
    }
    
}