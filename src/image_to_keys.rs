use crate::average_key_brightnesses::KeyBrightnesses;
use image::{imageops::resize, DynamicImage, GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

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

pub fn keys_to_image(keys: &[String], font: Font, scale: Scale) -> Result<GrayImage, String> {
    const KEY_COLOR: Luma<u8> = Luma([255]);
    let (image_row_width, image_row_height) = text_size(
        scale,
        &font,
        keys.get(0).ok_or("The keys vector is empty.")?,
    );
    let mut image = GrayImage::new(
        image_row_width as u32,
        image_row_height as u32 * <u32>::try_from(keys.len()).unwrap(),
    );
    for (y, key) in keys.iter().enumerate() {
        draw_text_mut(
            &mut image,
            KEY_COLOR,
            0,
            y as i32 * image_row_height,
            scale,
            &font,
            key,
        );
    }
    Ok(image)
}

#[cfg(test)]
mod image_to_keys_tests {
    use super::*;

    #[test]
    fn find_closest_brightness() {
        let vector = [0u8, 2, 4];
        assert_eq!(GrayImage::closest_brightness(&vector, 1), Some(0));
        assert_eq!(GrayImage::closest_brightness(&[], 1), None);
    }
}
