use image::{GrayImage, Luma};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub const PRINTABLE_CHARACTERS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

pub struct AverageKeyBrightnesses {
    keys: String,
    brightnesses: Vec<u8>,
}

impl AverageKeyBrightnesses {
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
    #[test]
    fn average_brightness() {
        let image = draw_filled_rect(
            &GrayImage::new(100, 100),
            Rect::at(0, 0).of_size(100, 100),
            AverageKeyBrightnesses::KEY_COLOR,
        );
        assert_eq!(AverageKeyBrightnesses::average_brightness(&image), 255);
    }
