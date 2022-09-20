use rand::{thread_rng, Rng};
use std::ops::Range;

// Constants
pub const PI: f64 = std::f64::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    PI / (180.0 * radians)
}

pub fn random_num(range: Range<f64>) -> f64 {
    thread_rng().gen_range(range)
}
