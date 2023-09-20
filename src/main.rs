use std::fs;

use image::io::Reader;

use crate::{
    as_string::AsString, as_string_options::AsStringOptions, char_brightnesses::CharBrightnesses,
    mark_up_options::MarkUpOptions,
};
pub mod as_string;
pub mod as_string_options;
pub mod char_brightnesses;
pub mod mark_up_options;

fn main() {
    let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    //"/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf";

    let markup_options = MarkUpOptions::new();
    let as_string_options = AsStringOptions::new();
    let char_brightnesses = CharBrightnesses::new(chars, &markup_options);

    let image = Reader::open("io/output/box_doggo.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let image = image.resize(400, 400, image::imageops::FilterType::Lanczos3);
    let image_text = image
        .to_luma8()
        .as_string(&char_brightnesses, &as_string_options);
    fs::write("io/output/text_doggo.txt", &image_text).unwrap();
    println!("{}", image_text);
}
