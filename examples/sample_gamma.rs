use std::ops::RangeInclusive;

use tinyrand::Wyrand;

use metromc::gamma::Gamma;
use metromc::sampler::{Config, Sampler};

fn main() {
    const SHAPE: f64 = 2.0;
    const RATE: f64 = 0.5;
    const RANGE: RangeInclusive<f64> = 0.0..=16.0;
    const NUM_BUCKETS: usize = 100;

    let sampler = Sampler::new(Config {
        rand: Wyrand::default(),
        dist: Gamma::new(SHAPE, RATE),
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