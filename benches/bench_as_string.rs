#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use std::num::NonZeroU32;

    use super::*;
    use char_art::{as_string::AsString, char_brightnesses::CharBrightnesses};
    use fast_image_resize::{FilterType, Image, PixelType, Resizer};
    use image::{imageops::resize, io::Reader, GrayImage, Luma};
    use rusttype::{Font, Scale};
    use test::{black_box, Bencher};

    fn resize_fast(image: &GrayImage, char_brightnesses_lut: &CharBrightnesses) {
        const HEIGHT_SHRINK_AMOUNT: u8 = 2;
        let self_width = image.width();
        let self_height = image.height();
        let source_image = Image::from_vec_u8(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height).unwrap(),
            image.clone().into_raw(),
            PixelType::U8,
        )
        .unwrap();

        let mut target_image = Image::new(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height / HEIGHT_SHRINK_AMOUNT as u32).unwrap(),
            source_image.pixel_type(),
        );

        let mut resizer = Resizer::new(fast_image_resize::ResizeAlg::Convolution(
            FilterType::Lanczos3,
        ));
        resizer
            .resize(&source_image.view(), &mut target_image.view_mut())
            .unwrap();
    }

    fn resize_default(image: &GrayImage, char_brightnesses_lut: &CharBrightnesses) {
        resize(
            image,
            image.width(),
            image.height() / 2,
            image::imageops::FilterType::Lanczos3,
        );
    }

    fn as_string_slow(image: &GrayImage, char_brightnesses_lut: &CharBrightnesses) -> String {
        const HEIGHT_SHRINK_AMOUNT: u8 = 2;
        let self_width = image.width();
        let self_height = image.height();
        let source_image = Image::from_vec_u8(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height).unwrap(),
            image.clone().into_raw(),
            PixelType::U8,
        )
        .unwrap();

        let mut target_image = Image::new(
            NonZeroU32::new(self_width).unwrap(),
            NonZeroU32::new(self_height / HEIGHT_SHRINK_AMOUNT as u32).unwrap(),
            source_image.pixel_type(),
        );

        let mut resizer = Resizer::new(fast_image_resize::ResizeAlg::Convolution(
            FilterType::Lanczos3,
        ));
        resizer
            .resize(&source_image.view(), &mut target_image.view_mut())
            .unwrap();

        let image_bytes = target_image.buffer();
        let mut char_buffer = String::with_capacity(image_bytes.len() + self_height as usize/2 + 2);
        for index in 0..image_bytes.len() {
            char_buffer.push(char_brightnesses_lut[image_bytes[index]]);
            if (index + 1) % self_width as usize == 0 {
                char_buffer.push('\n');
            }
        }
        char_buffer
    }

    #[bench]
    fn bench_resize_fast(b: &mut Bencher) {
        let char_map = CharBrightnesses::default();
        let image = Reader::open("io/output/box_doggo.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma8();
        b.iter(|| {
            resize_fast(black_box(&image), black_box(&char_map));
        });
    }

    #[bench]
    fn bench_resize_default(b: &mut Bencher) {
        let char_map = CharBrightnesses::default();
        let image = Reader::open("io/output/box_doggo.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma8();
        b.iter(|| {
            resize_default(black_box(&image), black_box(&char_map));
        });
    }

    #[bench]
    fn bench_as_string(b: &mut Bencher) {
        let char_map = CharBrightnesses::default();
        let image = Reader::open("io/output/smol_box_doggo.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma8();
        b.iter(|| {
            black_box(&image).as_string(black_box(&char_map));
        });
    }

    #[bench]
    fn bench_as_string_slow(b: &mut Bencher) {
        let char_map = CharBrightnesses::default();
        let image = Reader::open("io/output/box_doggo.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma8();
        b.iter(|| {
            as_string_slow(black_box(&image), black_box(&char_map));
        });
    }
}
