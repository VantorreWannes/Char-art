#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use test::{Bencher, black_box};

    #[bench]
    fn bench_pow(b: &mut Bencher) {
        b.iter(|| {
            let x: f64 = 211.0 * 11.0;
            let y: f64 = 301.0 * 103.0;

            for _ in 1..100 {
                black_box(x.powf(y).powf(x));
            }
        });
    }
}