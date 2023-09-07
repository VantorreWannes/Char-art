use std::ops::Index;

use rusttype::{point, Font, Scale, ScaledGlyph};

pub const CHARS_LENGTH: usize = 95;
pub const CHARS: [char; CHARS_LENGTH] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
    'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
];
const FONT: &[u8] = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
const SCALE: f32 = 40.0;
const COLOR: f32 = u8::MAX as f32;

pub struct BrightnessCharMap {
    char_lut: [char; u8::MAX as usize],
}

impl BrightnessCharMap {

    fn new() -> BrightnessCharMap {
        let brightnesses_tuples = Self::get_brightness_tuples();
        Self {
            char_lut: Self::brightness_tuples_to_lut(brightnesses_tuples),
        }
    }

    fn get_brightness_tuples() -> [(char, u8); CHARS_LENGTH] {
        let mut brightnesses = [(' ', u8::MIN); CHARS.len()];
        let font = Font::try_from_bytes(FONT).unwrap();
        let scale = Scale::uniform(SCALE);
        for (i, char) in CHARS.into_iter().enumerate() {
            unsafe {
                *brightnesses.get_unchecked_mut(i) = (
                    char,
                    Self::average_brightness(font.glyph(char).scaled(scale)),
                );
            }
        }
        brightnesses
    }

    fn average_brightness(glyph: ScaledGlyph) -> u8 {
        let mut buffer = [u32::MIN; 32];
        let total_pixels = Self::glyph_width(&glyph) * SCALE;
        glyph.positioned(point(0.0, 0.0)).draw(|x, y, v| {
            if v > 0.5 {
                buffer[y as usize] |= 1 << x;
            }
        });
        let brightness = buffer
            .into_iter()
            .map(|row| row.count_ones() as u16)
            .sum::<u16>();
        ((brightness as f32 * COLOR) / total_pixels) as u8
    }

    fn glyph_width(glyph: &ScaledGlyph) -> f32 {
        let h_metrics = glyph.h_metrics();
        h_metrics.advance_width + h_metrics.left_side_bearing
    }

    fn brightness_tuples_to_lut(tuples: [(char, u8); CHARS_LENGTH]) -> [char; u8::MAX as usize] {
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

    ///# Safety
    ///Can't fail if self.char_lut is length 255 or longer.
    ///Which it always is.
    pub unsafe fn get_unchecked(&self, brigthness: u8) -> char {
        *self.char_lut.get_unchecked(brigthness as usize)
    }
}

impl Default for BrightnessCharMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for BrightnessCharMap {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.char_lut[index]
    }
}

#[cfg(test)]
mod brightness_char_map_tests {
    use super::*;

    #[test]
    fn get_brightness_tuples() {
        let _char_map = BrightnessCharMap::default();
    }
}
