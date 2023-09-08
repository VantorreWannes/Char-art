use crate::brightness_char_map::BrightnessCharMap;
use image::{imageops::resize, DynamicImage, GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub trait AsChars {
    const HEIGHT_SHRINK_AMOUNT: u32;
    fn as_chars(&self, char_map: &BrightnessCharMap) -> String;
}

impl AsChars for GrayImage {
    const HEIGHT_SHRINK_AMOUNT: u32 = 2;

    fn as_chars(&self, char_map: &BrightnessCharMap) -> String {
        let image = resize(
            self,
            self.width(),
            self.height() / Self::HEIGHT_SHRINK_AMOUNT,
            image::imageops::FilterType::Lanczos3,
        );

        let image_width = image.width() as usize;
        let mut char_image = String::with_capacity(image.len() + image.height() as usize + 2);
        for (index, brightness) in image.iter().enumerate() {
            unsafe {
                char_image.push(char_map.get_unchecked(*brightness));
            };
            if (index + 1) % image_width == 0 {
                char_image.push('\n');
            }
        }
        char_image
    }
}

impl AsChars for DynamicImage {
    const HEIGHT_SHRINK_AMOUNT: u32 = 2;

    fn as_chars(&self, char_map: &BrightnessCharMap) -> String {
        self.to_luma8().as_chars(char_map)
    }
}

pub fn as_chars_image(chars: &str, font: &Font, scale: Scale) -> GrayImage {
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
mod brightness_char_map_tests {
    use image::{imageops::FilterType, io::Reader};

    use super::*;

    #[test]
    fn as_chars() {
        let char_map = BrightnessCharMap::default();
        let image = Reader::open("images/output/doggo_small.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let image = image.as_chars(&char_map);
        println!("{}", image);
    }

    #[test]
    fn smaller_image() {
        let image = Reader::open("images/input/doggo.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let resize_amount = 5u32;
        image
            .resize(
                image.width() / resize_amount,
                image.height() / resize_amount,
                FilterType::Lanczos3,
            )
            .save("images/output/doggo_small.jpg")
            .unwrap();
    }

    #[test]
    fn const_image() {
        let image = Reader::open("images/input/doggo.jpg")
            .unwrap()
            .decode()
            .unwrap();
        image
            .resize(1000, 1000, FilterType::Lanczos3)
            .save("images/output/doggo_const.jpg")
            .unwrap();
    }
}
