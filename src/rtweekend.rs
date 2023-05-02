use derive_more::*;
use rand::{thread_rng, Rng};
use std::ops::{Range, RangeInclusive};

// Constants
pub const PI: f64 = std::f64::consts::PI;

/// Angle's amplitude in degrees
#[derive(Clone, Copy, PartialEq, From, Add, Sub, Mul, Div)]
pub struct Degrees(f64);

impl Degrees {
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<Radians> for Degrees {
    /// Convert from radians to degrees
    fn from(radians: Radians) -> Self {
        Degrees(PI / (180.0 * radians.0))
    }
}

/// Angle's amplitude in radians
#[derive(Clone, Copy, PartialEq, From, Add, Sub, Mul, Div)]
pub struct Radians(f64);

impl Radians {
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<Degrees> for Radians {
    /// Convert from degrees to radians
    fn from(degrees: Degrees) -> Self {
        Radians(degrees.0 * PI / 180.0)
    }
}

/// Return a random number in the range [0.0, 1.0)
pub fn random_num() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

/// Return a random number in the given range
pub fn random_num_in_range(range: Range<f64>) -> f64 {
    thread_rng().gen_range(range)
}

/// - If `elem` is inside the given range, return `elem`.
/// - If `elem` is smaller than the given range, return the smallest element in that range.
/// - If `elem` is greater than the given range, return the greatest element in that range.
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
