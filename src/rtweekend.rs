use rand::{thread_rng, Rng};
use std::ops::{Range, RangeInclusive};

// Constants
pub const PI: f64 = std::f64::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    PI / (180.0 * radians)
}

pub fn random_num() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

pub fn random_num_in_range(range: Range<f64>) -> f64 {
    thread_rng().gen_range(range)
}

pub fn clamp<N>(elem: N, range: RangeInclusive<N>) -> N
where
    N: PartialOrd + Copy,
{
    if &elem < range.start() {
        *range.start()
    } else if &elem > range.end() {
        *range.end()
    } else {
        elem
    }
}
