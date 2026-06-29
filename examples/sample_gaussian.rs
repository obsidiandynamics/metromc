use std::ops::RangeInclusive;
use tinyrand::Wyrand;
use metromc::gaussian::Gaussian;
use metromc::sampler::{Config, Sampler};

fn main() {
    const MEAN: f64 = 0.0;
    const STD_DEV: f64 = 1.0;
    const RANGE: RangeInclusive<f64> = -5.0..=5.0;
    const NUM_BUCKETS: usize = 100;

    let sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        dist: Gaussian::new(MEAN, STD_DEV),
        range: RANGE,
    });

    let mut buckets = [0_usize; NUM_BUCKETS];
    let span = RANGE.end() - RANGE.start();
    for sample in sampler.skip(100).take(1_000_000_000) {
        let bucket = usize::min(((sample - RANGE.start()) / span * NUM_BUCKETS as f64).round() as usize, NUM_BUCKETS - 1);
        buckets[bucket] += 1;
    }

    println!("bucket start, count");
    println!("------------, -----");

    for (bucket, count) in buckets.iter().enumerate() {
        let bucket_start = RANGE.start() + bucket as f64 / NUM_BUCKETS as f64 * span;
        println!("{bucket_start:.3}, {}", count);
    }
}