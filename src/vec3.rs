use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use crate::rtweekend::{random_num, random_num_in_range};

/// Struct that can either represent a 3D vector or a 3D point.
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new Vec3 from 3 coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create a new Vec3 with coordinates (0, 0, 0)
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Return a random vector for which `x`, `y` and `z` belong to the range [0.0, 1.0)
    pub fn random_vec() -> Vec3 {
        Vec3 {
            x: random_num(),
            y: random_num(),
            z: random_num(),
        }
    }

    /// Return a random vector for which `x`, `y` and `z` belong to the given range
    pub fn random_vec_with_range(range: Range<f64>) -> Vec3 {
        Vec3 {
            x: random_num_in_range(range.clone()),
            y: random_num_in_range(range.clone()),
            z: random_num_in_range(range),
        }
    }

    /// Return a random point in a unit sphere (sphere of radius=1)
    pub fn random_point_in_unit_sphere() -> Vec3 {
        loop {
            let point = Vec3::random_vec_with_range(-1.0..1.0);
            // length_squared() is faster than length(), and if length() < 1, then length_squared() < 1
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    /// Return a random unit vector
    pub fn random_unit_vec() -> Vec3 {
        Vec3::random_point_in_unit_sphere().unit_vec()
    }

    /// Return the squared length of this vector
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Return the length of this vector
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Multiply this vector with another one using the dot product
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Multiply this vector with another one using the cross product
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Return the unit vector with the same direction as this one
    pub fn unit_vec(self) -> Vec3 {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
