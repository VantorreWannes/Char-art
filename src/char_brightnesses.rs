use std::ops::Index;

use image::Luma;
use rusttype::{point, Font, Scale, ScaledGlyph};

use crate::mark_up_options::MarkUpOptions;

#[derive(Debug)]
pub struct CharBrightnesses {
    char_lut: [char; u8::MAX as usize],
}

impl CharBrightnesses {
    pub fn new(chars: &str, options: &MarkUpOptions) -> Self {
        let (font, scale, color) = options.get_values();
        let brightnesses_tuples = Self::get_brightness_tuples(chars, &font, &scale, &color);
        CharBrightnesses {
            char_lut: Self::brightness_tuples_to_lut(brightnesses_tuples),
        }
    }

    pub fn get_brightness_tuples(
        chars: &str,
        font: &Font,
        scale: &Scale,
        color: &Luma<u8>,
    ) -> Vec<(char, u8)> {
        let mut brightnesses = vec![(' ', u8::MIN); chars.len()];
        let color = color.0[0] as f32;
        for (i, char) in chars.char_indices() {
            unsafe {
                *brightnesses.get_unchecked_mut(i) = (
                    char,
                    Self::average_brightness(font.glyph(char).scaled(*scale), &color),
                );
            }
        }

        brightnesses
    }

    fn average_brightness(glyph: ScaledGlyph, color: &f32) -> u8 {
        let (glyph_width, glyph_height) = Self::glyph_dimensions(&glyph);
        let buffer_column = vec![u8::MIN; glyph_height as usize];
        let mut buffer = vec![buffer_column; glyph_width as usize];
        let total_pixels = glyph_width * glyph_height;
        glyph.positioned(point(0.0, 0.0)).draw(|x, y, v| {
            if v > 0.5 {
                buffer[x as usize][y as usize] = 1u8;
            }
        });
        let brightness = buffer
            .into_iter()
            .map(|row| row.iter().sum::<u8>() as u16) //Only works if scale is <= then u8::MAX;
            .sum::<u16>();
        ((brightness as f32 * color) / total_pixels) as u8
    }

    fn glyph_dimensions(glyph: &ScaledGlyph) -> (f32, f32) {
        let h_metrics = glyph.h_metrics();
        (
            h_metrics.advance_width + h_metrics.left_side_bearing,
            glyph.scale().y,
        )
    }

    fn brightness_tuples_to_lut(tuples: Vec<(char, u8)>) -> [char; u8::MAX as usize] {
        let mut lut = ['\x00'; u8::MAX as usize];
        let mut offset = [u8::MAX; u8::MAX as usize];

        for (char, brightness) in tuples {
            lut[brightness as usize] = char;
            offset[brightness as usize] = 0u8;

            let mut i = 0usize;
            while i < brightness as usize {
                let new_offset = brightness - i as u8;
                if offset[i] < new_offset {
                    break;
                }
                unsafe {
                    *offset.get_unchecked_mut(i) = new_offset;
                    *lut.get_unchecked_mut(i) = char;
                }

                i += 1;
            }

            let mut i = (brightness + 1) as usize;
            while i < u8::MAX as usize {
                let new_offset = i as u8 - brightness;
                if offset[i] < new_offset {
                    break;
                }
                unsafe {
                    *offset.get_unchecked_mut(i) = new_offset;
                    *lut.get_unchecked_mut(i) = char;
                }
                i += 1;
            }
        }
        lut
    }
}

impl Default for CharBrightnesses {
    fn default() -> Self {
        const LUT: [char; 255usize] = [
            '`', '`', '.', '.', '\'', ':', '1', '1', ';', '-', '=', '=', '/', '_', '_', ')', '\\',
            '<', '~', '~', 'l', '}', '}', ']', ']', 'v', 'F', 'c', 'x', 'j', 's', 't', 'Z', 'Y',
            'f', 'o', 'y', 'z', 'N', 'S', 'u', 'u', 'e', 'G', 'w', 'M', 'M', 'p', 'q', 'q', 'Q',
            'B', 'B', 'B', 'B', 'g', 'g', 'm', 'm', 'm', 'm', 'm', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
        ];
        Self { char_lut: LUT }
    }
}

impl Index<u8> for CharBrightnesses {
    type Output = char;
    fn index(&self, index: u8) -> &Self::Output {
        unsafe { &self.char_lut.get_unchecked(index as usize) }
    }
}

#[cfg(test)]
mod char_brightnesses_tests {
    use crate::mark_up_options::MarkUpOptions;

    use super::CharBrightnesses;
    use image::Luma;
    use rusttype::{Font, Scale};

    #[test]
    fn glyph_dimensions() {
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        for scale in [10.0, 20.0, 30.0] {
            let scale = Scale::uniform(scale);
            for char in "abcdefghijklmnopqrstuvwxyz.=+-*/(){}[]<>:;\"'!@#$%^&*()_+|~".chars() {
                let ghlyph = font.glyph(char).scaled(scale);
                let (width, height) = CharBrightnesses::glyph_dimensions(&ghlyph);
                let glyph_h_metrics = ghlyph.h_metrics();
                assert_eq!(
                    width,
                    glyph_h_metrics.advance_width + glyph_h_metrics.left_side_bearing
                );
                assert_eq!(height, scale.y);
            }
        }
    }

    #[test]
    fn average_brightness() {
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        for scale in [10.0, 20.0, 30.0] {
            let scale = Scale::uniform(scale);
            let mut collection: Vec<u8> = Vec::with_capacity(3);
            for char in " aA".chars() {
                let glyph = font.glyph(char).scaled(scale);
                let brightness = CharBrightnesses::average_brightness(glyph, &255.0);
                collection.push(brightness);
            }

            for i in 0..collection.len() - 1 {
                assert!(collection[i] <= collection[i + 1]);
            }
        }
    }

    #[test]
    fn get_brightness_tuples() {
        let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        let scale = Scale::uniform(12.0);
        let color = Luma([255]);
        let brightness_tuples =
            CharBrightnesses::get_brightness_tuples(chars, &font, &scale, &color);
        assert_eq!(brightness_tuples.len(), chars.len());
    }

    #[test]
    fn brightness_tuples_to_lut() {
        let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        let scale = Scale::uniform(12.0);
        let color = Luma([255]);
        let brightness_tuples =
            CharBrightnesses::get_brightness_tuples(chars, &font, &scale, &color);
        let char_lut = CharBrightnesses::brightness_tuples_to_lut(brightness_tuples.clone());
        assert_eq!(char_lut.len(), 255);
    }

    #[test]
    fn new() {
        let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        let scale = Scale::uniform(12.0);
        let color = Luma([255]);
        let char_brightnesses = CharBrightnesses::new(chars, &MarkUpOptions::default());
        dbg!(char_brightnesses);
    }

    #[test]
    fn index() {
        let char_brightnesses = CharBrightnesses::default();
        for i in 0..=255u8 {
            char_brightnesses[i];
        }
    }
}
