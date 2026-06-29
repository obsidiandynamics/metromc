use std::ops::RangeInclusive;
use tinyrand::Rand;
use crate::Pdf;

pub struct Config<R: Rand, P: Pdf> {
    pub rand: R,
    pub dist: P,
    pub range: RangeInclusive<f64>,
}

pub struct Sampler<R: Rand, P: Pdf> {
    config: Config<R, P>,
    current: f64,
    prob_current: f64,
    span: f64,
}
impl<R: Rand, P: Pdf> Sampler<R, P> {
    pub fn new(config: Config<R, P>) -> Self {
        let span  = config.range.end() - config.range.start();
        let current = (config.range.start() + config.range.end()) / 2.0;
        let prob_current = config.dist.prob(current);
        Self {
            config,
            current,
            prob_current,
            span,
        }
    }

    #[inline(always)]
    pub fn next_sample(&mut self) -> f64 {
        let next = random_f64(&mut self.config.rand) * self.span + self.config.range.start();
        // let next = random_f64(&mut self.config.rand) * self.span - self.span / 2.0 + self.current;
        let prob_next = self.config.dist.prob(next);
        //println!("current: {:.3}, prob_current: {:.3}, next: {next:.3}, prob_next: {prob_next:.3}", self.current, self.prob_current);
        if prob_next >= self.prob_current {
            self.current = next;
            self.prob_current = prob_next;
        } else {
            let threshold = prob_next / self.prob_current;
            let rand = random_f64(&mut self.config.rand);
            if rand < threshold {
                self.current = next;
                self.prob_current = prob_next;
            }
        }
        self.current
    }
}

impl<R: Rand, P: Pdf> Iterator for Sampler<R, P> {
    type Item = f64;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_sample())
    }
}

#[inline(always)]
fn random_f64(rand: &mut impl Rand) -> f64 {
    rand.next_u64() as f64 / u64::MAX as f64
}