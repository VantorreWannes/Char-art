use crate::{
    as_string_options::AsStringOptions, char_brightnesses::CharBrightnesses,
    mark_up_options::MarkUpOptions,
};
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use image::{io::Reader, DynamicImage, GenericImageView, GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::num::NonZeroU32;

pub trait AsString {
    const HEIGHT_SHRINK_AMOUNT: u8;
    fn as_string(
        &self,
        char_brightnesses_lut: &CharBrightnesses,
        options: &AsStringOptions,
    ) -> String;
}

impl AsString for GrayImage {
    const HEIGHT_SHRINK_AMOUNT: u8 = 2;

    fn as_string(
        &self,
        char_brightnesses_lut: &CharBrightnesses,
        options: &AsStringOptions,
    ) -> String {
        let (shrink, darken) = options.get_values();
        let width = self.width();
        let height = self.height();

        let image = Image::from_vec_u8(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
            self.clone().into_raw(),
            PixelType::U8,
        )
        .unwrap();

        let mut target_image = Image::new(
            NonZeroU32::new(width / shrink).unwrap(),
            NonZeroU32::new((height / shrink) / Self::HEIGHT_SHRINK_AMOUNT as u32).unwrap(),
            image.pixel_type(),
        );

        let mut resizer = Resizer::new(fast_image_resize::ResizeAlg::Convolution(
            FilterType::Lanczos3,
        ));
        resizer
            .resize(&image.view(), &mut target_image.view_mut())
            .ok()
            .unwrap();

        let image_bytes = target_image.buffer();
        let mut char_buffer = String::with_capacity(
            image_bytes.len() + ((height / shrink) as usize / Self::HEIGHT_SHRINK_AMOUNT as usize),
        );
        dbg!(char_buffer.capacity());
        for index in 0..image_bytes.len() {
            char_buffer.push(char_brightnesses_lut[image_bytes[index].saturating_sub(darken)]);
            if (index + 1) % width as usize == 0 {
                char_buffer.push('\n');
            }
        }
        dbg!(char_buffer.capacity(), char_buffer.len());
        char_buffer
    }
}

impl AsString for DynamicImage {
    const HEIGHT_SHRINK_AMOUNT: u8 = 2;
    fn as_string(
        &self,
        char_brightnesses_lut: &CharBrightnesses,
        options: &AsStringOptions,
    ) -> String {
        self.to_luma8().as_string(char_brightnesses_lut, options)
    }
}

pub fn string_to_image(chars: &str, options: &MarkUpOptions) -> GrayImage {
    let (font, scale, color) = options.get_values();
    let rows = chars.split('\n').collect::<Vec<&str>>();
    let text_size = text_size(scale, &font, rows[0]);
    let mut image = GrayImage::new(text_size.0 as u32, text_size.1 as u32 * rows.len() as u32);
    for (y, line) in rows.iter().enumerate() {
        draw_text_mut(
            &mut image,
            color,
            0,
            text_size.1 * y as i32,
            scale,
            &font,
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
        let image_text = image
            .to_luma8()
            .as_string(&char_map, &AsStringOptions::default());
        fs::write("io/output/text_doggo.txt", image_text).unwrap();
    }
}
