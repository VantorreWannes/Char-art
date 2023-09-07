use std::{
    fs::{self, File},
    io,
};

use clap::{arg, value_parser, Arg, ArgMatches, Command};
use image::{imageops::FilterType, io::Reader, DynamicImage};
use rusttype::{Font, Scale};

pub mod as_chars;
pub mod brightness_char_map;

fn get_matches() -> ArgMatches {
    Command::new("char_art")
        .args(&[
            arg!(-i --image <Path> "Input image path")
                .required(true)
                .value_parser(value_parser!(String)),
            arg!(-s --shrink [u32] "Resize divide amount").value_parser(value_parser!(u32)),
            arg!(-d --darken [i32] "Darken amount (input negative values to brighten)")
                .value_parser(value_parser!(i32)),
        ])
        .subcommand(
            Command::new("to_image")
                .about("Convert the converted image back into an image.")
                .args(&[
                    arg!(-i --image <Path> "output image path")
                        .required(true)
                        .value_parser(value_parser!(String)),
                    arg!(-f --font [Path] "Font path").value_parser(value_parser!(String)),
                    arg!(-s --size [f32] "Text scale amount").value_parser(value_parser!(f32)),
                ]),
        )
        .get_matches()
}

fn get_path(matches: &ArgMatches) -> Result<String, image::ImageError> {
    match matches.get_one::<String>("image") {
        Some(path) => Ok(path.to_string()),
        None => Err(image::ImageError::IoError(io::Error::from(
            io::ErrorKind::NotFound,
        ))),
    }
}

fn get_image(matches: &ArgMatches) -> Result<DynamicImage, image::ImageError> {
    let path = get_path(matches)?;
    Reader::open(path)?.decode()
}

fn shrink_image(image: DynamicImage, amount: Option<&u32>) -> DynamicImage {
    match amount {
        Some(amount) => image.resize(
            image.width() / amount,
            image.height() / amount,
            FilterType::Lanczos3,
        ),
        None => image,
    }
}

fn darken_image(image: DynamicImage, amount: Option<&i32>) -> DynamicImage {
    match amount {
        Some(amount) => image.brighten(-*amount),
        None => image,
    }
}

fn get_font(matches: &ArgMatches) -> Result<Vec<u8>, image::ImageError> {
    match matches.get_one::<String>("font") {
        Some(path) => Ok(fs::read(path)?),
        None => {
            Ok(include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf").to_vec())
        }
    }
}

fn get_scale(matches: &ArgMatches) -> Result<Scale, image::ImageError> {
    const DEFAULT_SCALE: f32 = 12.0;

    match matches.get_one::<f32>("size") {
        Some(size) => Ok(Scale::uniform(*size)),
        None => Ok(Scale::uniform(DEFAULT_SCALE)),
    }
}

fn handle_sub_matches<'a>(
    image: &'a mut DynamicImage,
    sub_matches: &'a ArgMatches,
) -> Result<DynamicImage, image::ImageError> {
    let path = get_path(sub_matches)?;
    let font = Font::try_from_vec(get_font(sub_matches)?)
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?;
    let scale = get_scale(sub_matches)?;

    Ok(())
}

fn main() -> Result<(), image::ImageError> {
    let matches = get_matches();
    let mut image = get_image(&matches)?;

    image = shrink_image(image, matches.get_one::<u32>("shrink"));
    image = darken_image(image, matches.get_one::<i32>("darken"));

    if let Some(sub_matches) = matches.subcommand_matches("to_image") { 
        handle_sub_matches(&mut image, sub_matches)?;
    }

    Ok(())
}
