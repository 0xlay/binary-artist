use ascii::{AsciiChar, AsciiString};
use image::{imageops, DynamicImage, GenericImageView, ImageError};

const SYMBOLS: [char; 14] = [
    '@', '#', '$', '%', '?', '*', '+', ';', ':', ',', '.', '|', '\\', '/',
];

pub struct ImgSize {
    pub width: u32,
    pub height: u32,
}

impl ImgSize {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

pub struct Converter {
    img_size: ImgSize,
    path: String,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            img_size: ImgSize::new(),
            path: String::new(),
        }
    }

    pub fn set_img(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn resize(&mut self, size: ImgSize) -> &mut Self {
        self.img_size = size;
        self
    }

    pub fn execute(&mut self) -> Result<AsciiString, ImageError> {
        match image::open(&self.path) {
            Ok(img) => {
                let small_img = img.resize(
                    self.img_size.width,
                    self.img_size.height,
                    imageops::FilterType::Lanczos3,
                );

                let gray_img = small_img.grayscale();

                Ok(convert_to_ascii(&gray_img))
            }
            Err(e) => Err(e),
        }
    }
}

fn convert_to_ascii(img: &DynamicImage) -> AsciiString {
    let mut ascii_art = AsciiString::new();
    let (width, height) = img.dimensions();
    let pixels = img.to_rgba8().into_vec();

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let r = pixels[idx] as f32;
            let g = pixels[idx + 1] as f32;
            let b = pixels[idx + 2] as f32;
            let brightness = (0.2126 * r + 0.7152 * g + 0.0722 * b) as u8;
            let ascii_char =
                AsciiChar::from_ascii(brightness_to_char(brightness)).unwrap_or(AsciiChar::Space);
            ascii_art.push(ascii_char);
        }
        ascii_art.push(AsciiChar::LineFeed);
    }

    ascii_art
}

fn brightness_to_char(brightness: u8) -> u8 {
    let idx = ((brightness as f32 / 255.0) * (SYMBOLS.len() - 1) as f32) as usize;
    SYMBOLS[idx] as u8
}
