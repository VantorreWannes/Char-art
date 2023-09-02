use crate::average_key_brightnesses::KeyBrightnesses;
use image::{DynamicImage, GrayImage, imageops::resize};


pub trait ImageToKeys {
    fn find_closest_brightness(vector: &[u8], input: u8) -> Option<u8> {
        vector
            .iter()
            .min_by_key(|&x| (*x as i32 - input as i32).abs())
            .copied()
    }
    fn as_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<Vec<String>, String>;
}

impl ImageToKeys for GrayImage {
    fn as_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<Vec<String>, String> {
        let image = resize(
            self, self.width(), self.height() / KeyBrightnesses::KEY_WIDTH_MULTIPLIER as u32, image::imageops::FilterType::Gaussian
        );
        let brightnesses = key_brightnesses.brightnesses();
        let brightness_key_map = key_brightnesses.as_hash_map();
        let mut keys = Vec::with_capacity(image.height().try_into().unwrap());
        for row in image.rows() {
            let mut keys_string = String::with_capacity(image.width().try_into().unwrap());
            for pixel in row {
                let closest_key_brightness =
                    Self::find_closest_brightness(brightnesses, pixel.0[0])
                        .ok_or("struct instance KeyBrightness is empty.".to_string())?;
                keys_string.push(
                    *brightness_key_map
                        .get(&closest_key_brightness)
                        .ok_or("Invalid brightness key.".to_string())?,
                );
            }
            keys.push(keys_string);
        }
        Ok(keys)
    }
}

impl ImageToKeys for DynamicImage {
    fn as_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<Vec<String>, String> {
        self.to_luma8().as_keys(key_brightnesses)
    }
}

#[cfg(test)]
mod image_to_keys_tests {
    use image::{imageops::FilterType, io::Reader};

    use super::*;

    #[test]
    fn find_closest_brightness() {
        let vector = [0u8, 2, 4];
        assert_eq!(GrayImage::find_closest_brightness(&vector, 1), Some(0));
        assert_eq!(GrayImage::find_closest_brightness(&[], 1), None);
    }

    #[test]
    fn image_to_keys() {
        let mut image = Reader::open("input/cool_cat.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let downscale_amount = 20;
        image = image.resize(
            image.width() / downscale_amount,
            image.height() / downscale_amount,
            FilterType::Nearest,
        );
        let key_brightnesses = KeyBrightnesses::default();
        let keys = image.as_keys(&key_brightnesses).unwrap();
        println!("{}", keys.join("\n"));
    }
}
