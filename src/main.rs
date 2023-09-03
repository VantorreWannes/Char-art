use image::{io::Reader, imageops::FilterType, DynamicImage};
use rusttype::{Font, Scale};
use crate::{average_key_brightnesses::KeyBrightnesses, image_to_keys::ImageToKeys};
pub mod average_key_brightnesses;
pub mod image_to_keys;

const PRINTABLE_CHARACTERS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[allow(dead_code)]
fn image_to_keys_custom(image: &mut DynamicImage) {
    let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
    let key_brightnesses = KeyBrightnesses::new(PRINTABLE_CHARACTERS, Font::try_from_bytes(font_bytes).unwrap(), Scale::uniform(30.0));
    let keys = image.to_luma8().as_keys(&key_brightnesses).unwrap();
    println!("{}", keys.join("\n"));
}

#[allow(dead_code)]
fn image_to_keys_default(image: &mut DynamicImage) {
    let keys = image.to_luma8().as_keys(&KeyBrightnesses::default()).unwrap();
    println!("{}", keys.join("\n"));
}


fn main() {
    let mut image = Reader::open("input/skull.jpg")
            .unwrap()
            .decode()
            .unwrap();
    let downscale_amount = 4;
    image = image.resize(
        image.width() / downscale_amount,
        image.height() / downscale_amount,
        FilterType::Lanczos3,
    );
    image = image.brighten(-50);
    image_to_keys_default(&mut image);
}
