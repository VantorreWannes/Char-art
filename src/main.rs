use std::fs;

use crate::{average_key_brightnesses::KeyBrightnesses, image_to_keys::ImageToKeys};
use clap::{arg, value_parser, Command};
use image::{imageops::FilterType, io::Reader};
use image_to_keys::keys_to_image;
use rusttype::{Font, Scale};
pub mod average_key_brightnesses;
pub mod image_to_keys;

fn main() -> Result<(), image::ImageError> {
    let cmd = Command::new("key_art")
        .args(&[
            arg!(-p --path <Path> "File to be converted into keys.").required(true),
            arg!(-s --shrink <u32> "Amount the file size will be divided by.")
                .value_parser(value_parser!(u32)),
            arg!(-d --darken <i32> "Amount to darken the image by.")
                .value_parser(value_parser!(i32)),
            arg!(-b --brighten <i32> "Amount to brighten the image by.")
                .value_parser(value_parser!(i32)),
        ])
        .subcommand(Command::new("output").args(&[
            arg!(-o --output <Path> "Output file.").required(true),
            arg!(-f --font <Path> "Font file.").value_parser(value_parser!(String)),
            arg!(-s --scale <f32> "Scale factor.").value_parser(value_parser!(f32)),
        ]));

    let matches = cmd.get_matches();
    let path = matches.get_one::<String>("path").unwrap();
    let shrink = matches.get_one::<u32>("shrink");
    let darken = matches.get_one::<i32>("darken");
    let brighten = matches.get_one::<i32>("brighten");

    let mut image = Reader::open(path)?.decode()?;
    if let Some(shrink) = shrink {
        image = image.resize(
            image.width() / shrink,
            image.height() / shrink,
            FilterType::Lanczos3,
        );
    }
    if let Some(darken) = darken {
        image = image.brighten(-darken);
    }
    if let Some(brighten) = brighten {
        image = image.brighten(*brighten);
    }
    let keys = image.as_keys(&KeyBrightnesses::default()).unwrap();

    if let Some(output) = matches.subcommand_matches("output") {
        let output_path = output.get_one::<String>("output");
        let font_path = output.get_one::<String>("font");
        let scale = output.get_one::<f32>("scale");

        let font_bytes = match font_path {
            Some(path) => fs::read(path)?,
            None => {
                include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf").to_vec()
            }
        };
        let scale = match scale {
            Some(amount) => Scale::uniform(*amount),
            None => Scale::uniform(12.0),
        };
        let key_image = keys_to_image(
            &keys.as_slice(),
            Font::try_from_bytes(&font_bytes).unwrap(),
            scale,
        )
        .unwrap();
        key_image.save(output_path.unwrap()).unwrap();
    } else {
        println!("{}", keys.join("\n"));
    }

    Ok(())
}
