use brightness_char_map::BrightnessCharMap;
use image::io::Reader;

use crate::as_chars::AsChars;
pub mod as_chars;
pub mod brightness_char_map;

fn main() {
    let image = Reader::open("images/output/doggo_const.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let image = image.as_chars(&BrightnessCharMap::default());
    println!("{}", image);
}
