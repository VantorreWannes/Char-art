#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use char_art::char_brightnesses::CharBrightnesses;
    use image::Luma;
    use rusttype::{Font, Scale};
    use test::{Bencher, black_box};
    use super::*;

    #[bench]
    fn bench_get_brightness_tuples(b: &mut Bencher) {
        let chars = "abcdefghijklmnopqrstuvwxyz.=+-*/(){}[]<>:;\"'!@#$%^&*()_+|~";
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        let scale = Scale::uniform(12.0);
        let color = Luma([255]);
        b.iter(|| {
            for _ in 1..100 {
                CharBrightnesses::get_brightness_tuples(black_box(chars), black_box(&font), black_box(&scale), black_box(&color));
            }
        });
    }

    #[bench]
    fn bench_new(b: &mut Bencher) {
        let chars = "abcdefghijklmnopqrstuvwxyz.=+-*/(){}[]<>:;\"'!@#$%^&*()_+|~";
        let font_bytes = include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(font_bytes).unwrap();
        let scale = Scale::uniform(12.0);
        let color = Luma([255]);
        b.iter(|| {
            for _ in 1..100 {
                CharBrightnesses::new(black_box(chars), black_box(&font), black_box(&scale), black_box(&color));
            }
        });
    }

    #[bench]
    fn bench_default(b: &mut Bencher) {        
        b.iter(|| {
            for _ in 1..100 {
                CharBrightnesses::default();
            }
        });
    }

    #[bench]
    fn bench_index(b: &mut Bencher) {
        let char_brightnesses = CharBrightnesses::default();
        
        b.iter(|| {
            for _ in 1..3921 {
                for i in 0..=255u8 {
                    char_brightnesses[i];
                }
            }
        });
    }
}