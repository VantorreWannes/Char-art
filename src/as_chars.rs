use std::str::FromStr;

use image::{imageops::resize, DynamicImage, GrayImage};

use crate::brightness_char_map::BrightnessCharMap;

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
        let mut char_image = String::with_capacity(image.len() + image.height() as usize+2);
        for (index, brightness) in image.iter().enumerate() {
            unsafe {
                char_image.push(char_map.get_unchecked(*brightness));
            };
            if (index + 1) % image_width == 0 {
                char_image.push('\n');
            }
        }
        char_image

        /*
        let total_pixels = image.len() + image.height() as usize;
        let mut char_image = vec!['\x00'; total_pixels - 1];
        let mut index = 0;
        let mut newline_amount = 0;
        while (index + image.height() as usize) < total_pixels {
            unsafe {
                let pixel = image.get_unchecked(index);
                *char_image.get_unchecked_mut(index + newline_amount) =
                    char_map.get_unchecked(*pixel);
                if (index + 1) % (image.width() as usize) == 0 {
                    *char_image.get_unchecked_mut(index + newline_amount) = '\n';
                    newline_amount += 1;
                }
                index += 1;
            }
        }

        String::from_iter(char_image)
         */
    }
}

impl AsChars for DynamicImage {
    const HEIGHT_SHRINK_AMOUNT: u32 = 2;

    fn as_chars(&self, char_map: &BrightnessCharMap) -> String {
        self.to_luma8().as_chars(char_map)
    }
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
