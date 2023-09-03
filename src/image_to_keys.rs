use crate::average_key_brightnesses::KeyBrightnesses;
use image::{imageops::resize, DynamicImage, GrayImage};

pub trait ImageToKeys {
    fn closest_brightness(vector: &[u8], input: u8) -> Option<u8> {
        vector
            .iter()
            .min_by_key(|&x| (*x as i32 - input as i32).abs())
            .copied()
    }
    fn as_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<Vec<String>, String>;
}

impl ImageToKeys for GrayImage {
    fn as_keys(&self, key_brightnesses: &KeyBrightnesses) -> Result<Vec<String>, String> {
        let resized_image = resize(
            self,
            self.width(),
            self.height() / KeyBrightnesses::KEY_WIDTH_MULTIPLIER as u32,
            image::imageops::FilterType::Lanczos3,
        );

        let brightnesses = key_brightnesses.brightnesses();
        let brightness_key_map = key_brightnesses.as_hash_map();

        resized_image
            .rows()
            .map(|row| -> Result<String, String> {
                row.map(|pixel| -> Result<char, String> {
                    let closest_brightness = Self::closest_brightness(brightnesses, pixel.0[0])
                        .ok_or("Couldn't find any brightnesses.".to_string())?;
                    Ok(brightness_key_map[&closest_brightness])
                })
                .collect::<Result<String, String>>()
            })
            .collect::<Result<Vec<String>, String>>()
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
        assert_eq!(GrayImage::closest_brightness(&vector, 1), Some(0));
        assert_eq!(GrayImage::closest_brightness(&[], 1), None);
    }
}
