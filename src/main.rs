use std::{
    io,
    path::{Path, PathBuf}, num::NonZeroU32,
};

use char_art::as_string::AsString;
use clap::{arg, value_parser, ArgMatches, Command};
use fast_image_resize::{Image, PixelType};
use image::{DynamicImage, io::Reader, GrayImage};

pub mod as_string;
pub mod char_brightnesses;

fn get_matches() -> ArgMatches {
    Command::new("char_art")
        .args(&[
            arg!(-i --image <Image> "Path to input image.")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            arg!(-s --shrink [u32] "Shrink amount.").value_parser(value_parser!(u32)),
            arg!(-d --darken [i32] "Darken amount.").value_parser(value_parser!(i32)),
            arg!(-f --font [Font] "Path to font ttf file.").value_parser(value_parser!(PathBuf)),
            arg!(-c --color [Color] "Grayscale color value.").value_parser(value_parser!(u8)),
            arg!(-s --scale [Scale] "Text scale.").value_parser(value_parser!(f32)),
        ])
        .subcommand(
            Command::new("to_file").args(&[
                arg!(-i --jpg <Image> "Path of jpg file to write the output to.")
                    .value_parser(value_parser!(PathBuf))
                    .required_unless_present_any(["txt", "png"]),
                arg!(-t --png <File> "Path of png file to write the output to.")
                    .value_parser(value_parser!(PathBuf))
                    .required_unless_present_any(["txt", "jpg"]),
                arg!(-t --txt <File> "Path of txt file to write the output to.")
                    .value_parser(value_parser!(PathBuf))
                    .required_unless_present_any(["png", "jpg"]),
            ]),
        )
        .get_matches()
}

fn get_required(matches: &ArgMatches, id: &str) -> io::Result<PathBuf> {
    matches
        .get_one::<PathBuf>(id)
        .cloned()
        .ok_or(io::ErrorKind::NotFound.into())
}

fn get_image(matches: &ArgMatches) -> Result<DynamicImage, image::ImageError> {
    let path = get_required(matches, "image")?;
    Reader::open(path)?.decode()
}

fn shrink_image(image: GrayImage, amount: Option<&u32>) -> DynamicImage {
    match amount {
        Some(amount) => 
        {
            let width = image.width();
            let height = image.height();
            let image = Image::from_vec_u8(
                NonZeroU32::new(image.width()).unwrap(),
                NonZeroU32::new(image.height()).unwrap(),
                image.clone().into_raw(),
                PixelType::U8,
            )
            .unwrap();
           AsString::fast_resize(image, (width, height/2)).unwrap().into()
        },
        None => image,
    }
}

fn main() {}
