use crate::average_key_brightnesses::KeyBrightnesses;
use image::{DynamicImage, GrayImage};

pub trait ImageToKeys {
    fn find_closest_brightness(vector: &[u8], input: u8) -> Option<u8> {
        vector
            .iter()
            .min_by_key(|&x| (*x as i32 - input as i32).abs())
            .copied()
    }
    fn image_to_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<String, String>;
}

impl ImageToKeys for GrayImage {
    fn image_to_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<String, String> {
        let brightnesses = key_brightnesses.brightnesses();
        let brightness_key_map = key_brightnesses.as_hash_map();
        let mut keys = String::with_capacity(self.len());
        for row in self.rows() {
            for pixel in row {
                let closest_key_brightness =
                    Self::find_closest_brightness(brightnesses, pixel.0[0])
                        .ok_or("struct instance KeyBrightness is empty.".to_string())?;
                keys.push(
                    *brightness_key_map
                        .get(&closest_key_brightness)
                        .ok_or("Invalid brightness key.".to_string())?,
                );
            }
            keys.push('\n');
        }
        Ok(keys)
    }
}

impl ImageToKeys for DynamicImage {
    fn image_to_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<String, String> {
        self.to_luma8().image_to_keys(key_brightnesses)
    }
}

#[cfg(test)]
mod image_to_keys_tests {
    use image::{io::Reader, imageops::FilterType};

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
        let downscale_amount = 10;
        image = image.resize(image.width()/downscale_amount, image.height()/downscale_amount, FilterType::Gaussian);
        let key_brightnesses = KeyBrightnesses::default();
        let keys = image.image_to_keys(&key_brightnesses).unwrap();
        println!("{keys}");
    }
}
