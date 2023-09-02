use image::{GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub const PRINTABLE_CHARACTERS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

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
            let mut image = GrayImage::new(key_chunk_row_width as u32, key_chunk_row_height as u32*Self::KEY_REPETITION as u32);
            for y in 0..Self::KEY_REPETITION {
                draw_text_mut(&mut image, Self::KEY_COLOR, 0, y as i32*key_chunk_row_height, scale, &font, &key_chunk_row);
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
}

#[cfg(test)]
mod tests {
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
        assert_eq!(AverageKeyBrightnesses::average_brightness(&bright_image), 255);
    }

    #[test]
    fn test_keys_average_brightnesses() {
        let key_brightnesess = AverageKeyBrightnesses::keys_average_brightnesses(PRINTABLE_CHARACTERS, get_font(), Scale::uniform(12.0));
        dbg!(key_brightnesess);
    }

    #[test]
    fn test_new() {
        let scale = Scale::uniform(12.0);
        let font = get_font();
        let average_key_brightnesess = AverageKeyBrightnesses::new(PRINTABLE_CHARACTERS, font.clone(), scale);
        let key_brightnesess = AverageKeyBrightnesses::keys_average_brightnesses(PRINTABLE_CHARACTERS, font, scale);
        assert_eq!(average_key_brightnesess.brightnesses, key_brightnesess);
    }
}
