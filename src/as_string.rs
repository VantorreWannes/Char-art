use crate::char_brightnesses::CharBrightnesses;
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use image::{io::Reader, DynamicImage, GenericImageView, GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::num::NonZeroU32;

pub trait AsString {
    const HEIGHT_SHRINK_AMOUNT: u8;
    fn as_string(&self, char_brightnesses_lut: &CharBrightnesses) -> String;
    fn fast_resize<'a>(image: Image<'a >, dimensions: (u32, u32)) -> Option<Image<'a>> {
        let mut target_image = Image::new(
            NonZeroU32::new(dimensions.0)?,
            NonZeroU32::new(dimensions.1)?,
            image.pixel_type(),
        );

        let mut resizer = Resizer::new(fast_image_resize::ResizeAlg::Convolution(
            FilterType::Lanczos3,
        ));
        resizer
            .resize(&image.view(), &mut target_image.view_mut())
            .ok()?;
        Some(target_image)
    }
}

impl AsString for GrayImage {
    const HEIGHT_SHRINK_AMOUNT: u8 = 2;

    fn as_string(&self, char_brightnesses_lut: &CharBrightnesses) -> String {
        let width = self.width();
        let height = self.height();

        let image = Image::from_vec_u8(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
            self.clone().into_raw(),
            PixelType::U8,
        )
        .unwrap();

        let resized_image = Self::fast_resize(image,(width, height/Self::HEIGHT_SHRINK_AMOUNT as u32)).unwrap();
        let image_bytes = resized_image.buffer();
        let mut char_buffer =
            String::with_capacity(image_bytes.len() + height as usize / 2 + 2);
        for index in 0..image_bytes.len() {
            char_buffer.push(char_brightnesses_lut[image_bytes[index]]);
            if (index + 1) % width as usize == 0 {
                char_buffer.push('\n');
            }
        }
        char_buffer
    }

    
}

impl AsString for DynamicImage {
    const HEIGHT_SHRINK_AMOUNT: u8 = 2;
    fn as_string(&self, char_brightnesses_lut: &CharBrightnesses) -> String {
        self.to_luma8().as_string(char_brightnesses_lut)
    }
}

pub fn string_to_image(chars: &str, font: &Font, scale: Scale) -> GrayImage {
    let rows = chars.split('\n').collect::<Vec<&str>>();
    let text_size = text_size(scale, font, rows[0]);
    let mut image = GrayImage::new(text_size.0 as u32, text_size.1 as u32 * rows.len() as u32);
    for (y, line) in rows.iter().enumerate() {
        draw_text_mut(
            &mut image,
            Luma([255]),
            0,
            text_size.1 * y as i32,
            scale,
            font,
            line,
        )
    }
    image
}

#[cfg(test)]
mod as_string_tests {
    use super::*;
    use std::fs;

    #[test]
    fn as_string() {
        let char_map = CharBrightnesses::default();
        let image = Reader::open("io/output/box_doggo.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let image_text = image.to_luma8().as_string(&char_map);
        fs::write("io/output/text_doggo.txt", image_text).unwrap();
    }
}
