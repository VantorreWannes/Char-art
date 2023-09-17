use crate::char_brightnesses::CharBrightnesses;
use fast_image_resize::{FilterType, Image, PixelType, ResizeAlg, Resizer};
use image::{io::Reader, DynamicImage, GenericImageView, GrayImage};
use std::num::NonZeroU32;

pub trait AsString {
    const HEIGHT_SHRINK_AMOUNT: u8;
    fn as_string(&self, char_brightnesses_lut: &CharBrightnesses) -> String;
}

impl AsString for GrayImage {
    const HEIGHT_SHRINK_AMOUNT: u8 = 2;

    fn as_string(&self, char_brightnesses_lut: &CharBrightnesses) -> String {
        let self_width = self.width();
        let self_height = self.height();
        let source_image = Image::from_vec_u8(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height).unwrap(),
            self.clone().into_raw(),
            PixelType::U8,
        )
        .unwrap();

        let mut target_image = Image::new(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height / Self::HEIGHT_SHRINK_AMOUNT as u32).unwrap(),
            source_image.pixel_type(),
        );

        let mut resizer = Resizer::new(fast_image_resize::ResizeAlg::Convolution(
            FilterType::Lanczos3,
        ));
        resizer
            .resize(&source_image.view(), &mut target_image.view_mut())
            .unwrap();

        let image_bytes = target_image.buffer();
        let mut char_buffer =
            String::with_capacity(image_bytes.len() + self_height as usize / 2 + 2);
        for index in 0..image_bytes.len() {
            char_buffer.push(char_brightnesses_lut[image_bytes[index]]);
            if (index + 1) % self_width as usize == 0 {
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
