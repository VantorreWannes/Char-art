use std::collections::HashMap;

use image::{GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub const PRINTABLE_CHARACTERS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, PartialEq, Clone)]
pub struct AverageKeyBrightnesses {
    keys: String,
    brightnesses: Vec<u8>,
}

impl AverageKeyBrightnesses {
    pub const KEY_REPETITION: u8 = 3;
    pub const KEY_WIDTH_MULTIPLIER: u8 = 2;
    pub const CHUNK_WIDTH_KEY_AMOUNT: usize =
        (Self::KEY_REPETITION as u16 * Self::KEY_WIDTH_MULTIPLIER as u16) as usize;
    const KEY_COLOR: Luma<u8> = Luma([255]);

    pub fn new(keys: &str, font: Font, scale: Scale) -> Self {
        Self {
            keys: keys.to_string(),
            brightnesses: Self::keys_average_brightnesses(keys, font, scale),
        }
    }

    fn keys_average_brightnesses(keys: &str, font: Font, scale: Scale) -> Vec<u8> {
        let key_chunk_rows: Vec<String> = keys
            .chars()
            .into_iter()
            .map(|key| key.to_string().repeat(Self::CHUNK_WIDTH_KEY_AMOUNT))
            .collect();
        let mut key_brightnesess: Vec<u8> = Vec::with_capacity(keys.len());
        for key_chunk_row in key_chunk_rows.iter() {
            let (key_chunk_row_width, key_chunk_row_height) =
                text_size(scale, &font, key_chunk_row);
            let mut image = GrayImage::new(
                key_chunk_row_width as u32,
                key_chunk_row_height as u32 * Self::KEY_REPETITION as u32,
            );
            for y in 0..Self::KEY_REPETITION {
                draw_text_mut(
                    &mut image,
                    Self::KEY_COLOR,
                    0,
                    y as i32 * key_chunk_row_height,
                    scale,
                    &font,
                    &key_chunk_row,
                );
            }
            key_brightnesess.push(Self::average_brightness(&image));
        }
        key_brightnesess
    }

    fn average_brightness(image: &GrayImage) -> u8 {
        (image
            .pixels()
            .map(|pixel| pixel.0[0] as usize)
            .sum::<usize>()
            / image.len())
        .try_into()
        .unwrap()
    }

    pub fn brightnesses(&self) -> &Vec<u8> {
        &self.brightnesses
    }

    pub fn keys(&self) -> &str {
        &self.keys
    }

    pub fn as_tuple(&self) -> Vec<(u8, char)> {
        <Vec<(u8, char)>>::from(self)
    }

    pub fn as_hash_map(&self) -> HashMap<u8, char> {
        HashMap::<u8, char>::from(self)
    }
}

impl Default for AverageKeyBrightnesses {
    fn default() -> Self {
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        Self::new(
            PRINTABLE_CHARACTERS,
            Font::try_from_bytes(font_bytes).unwrap(),
            Scale::uniform(12.0),
        )
    }
}

impl From<&AverageKeyBrightnesses> for Vec<(u8, char)> {
    fn from(average_key_brightnesses: &AverageKeyBrightnesses) -> Self {
        average_key_brightnesses
            .brightnesses
            .iter()
            .copied()
            .zip(average_key_brightnesses.keys.chars())
            .map(|(brigthness, key)| (brigthness, key))
            .collect()
    }
}

impl From<&AverageKeyBrightnesses> for HashMap<u8, char> {
    fn from(average_key_brightnesses: &AverageKeyBrightnesses) -> Self {
        HashMap::from_iter(<Vec<(u8, char)>>::from(average_key_brightnesses).into_iter())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use image::{GrayImage, Luma};
    use imageproc::{drawing::draw_filled_rect, rect::Rect};
    use rusttype::{Font, Scale};
    use super::{AverageKeyBrightnesses, PRINTABLE_CHARACTERS};

    fn get_font() -> Font<'static> {
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        Font::try_from_bytes(font_bytes).unwrap()
    }

    #[test]
    fn average_brightness() {
        let bright_image = draw_filled_rect(
            &GrayImage::new(100, 100),
            Rect::at(0, 0).of_size(100, 100),
            AverageKeyBrightnesses::KEY_COLOR,
        );
        let dark_image = draw_filled_rect(
            &GrayImage::new(100, 100),
            Rect::at(0, 0).of_size(100, 100),
            Luma([0]),
        );
        assert_eq!(AverageKeyBrightnesses::average_brightness(&dark_image), 0);
        assert_eq!(
            AverageKeyBrightnesses::average_brightness(&bright_image),
            255
        );
    }

    #[test]
    fn keys_average_brightnesses() {
        let key_brightnesess = AverageKeyBrightnesses::keys_average_brightnesses(
            PRINTABLE_CHARACTERS,
            get_font(),
            Scale::uniform(12.0),
        );
    }

    #[test]
    fn new() {
        let scale = Scale::uniform(12.0);
        let font = get_font();
        let average_key_brightnesess =
            AverageKeyBrightnesses::new(PRINTABLE_CHARACTERS, font.clone(), scale);
        let key_brightnesess =
            AverageKeyBrightnesses::keys_average_brightnesses(PRINTABLE_CHARACTERS, font, scale);
        assert_eq!(average_key_brightnesess.brightnesses, key_brightnesess);
    }

    #[test]
    fn default() {
        let scale = Scale::uniform(12.0);
        let font = get_font();
        let average_key_brightnesess =
            AverageKeyBrightnesses::new(PRINTABLE_CHARACTERS, font.clone(), scale);
            assert_eq!(average_key_brightnesess, AverageKeyBrightnesses::default());
    }

    #[test]
    fn as_tuple() {
        let average_key_brightnesess = AverageKeyBrightnesses::default();
        assert_eq!(
            <Vec<(u8, char)>>::from(&average_key_brightnesess)[0],
            (
                average_key_brightnesess.brightnesses[0],
                average_key_brightnesess.keys.chars().next().unwrap()
            )
        )
    }

    #[test]
    fn as_hash_map() {
        let average_key_brightnesess = AverageKeyBrightnesses::default();
        let first_tuple = average_key_brightnesess.as_tuple()[0];
        let binding = <HashMap<u8, char>>::from(&average_key_brightnesess);
        let hash_map_tuple = binding.get_key_value(&first_tuple.0).unwrap();
        assert_eq!(
            hash_map_tuple, (&first_tuple.0, &first_tuple.1)
        )
    }
}
