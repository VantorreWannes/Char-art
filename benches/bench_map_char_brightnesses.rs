#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use char_art::{char_brightnesses::CharBrightnesses, as_string_options::AsStringOptions, mark_up_options::MarkUpOptions};
    use image::Luma;
    use rusttype::{Font, Scale};
    use test::{Bencher, black_box};
    use super::*;


    #[bench]
    fn bench_new(b: &mut Bencher) {
        let chars = "abcdefghijklmnopqrstuvwxyz.=+-*/(){}[]<>:;\"'!@#$%^&*()_+|~";
        let options = MarkUpOptions::default();
        b.iter(|| {
            for _ in 1..100 {
                CharBrightnesses::new(chars, &options);
            }
        });
    }

    #[bench]
    fn bench_default(b: &mut Bencher) {
        let chars = "abcdefghijklmnopqrstuvwxyz.=+-*/(){}[]<>:;\"'!@#$%^&*()_+|~";
        let options = MarkUpOptions::default();
        b.iter(|| {
            for _ in 1..100 {
                CharBrightnesses::default();
            }
        });
    }
}