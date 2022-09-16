//mod crate::vec3;
use crate::vec3::Vec3;

/// Struct that represents a ray.
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray given an origin point and a direction vector, for
    /// which the unit vector will be calculated and saved.
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.unit_vec(),
        }
    }

    // Point where the ray is at given a distance from its origin point.
    pub fn at(self, distance: f64) -> Vec3 {
        self.origin + distance * self.direction
    }
}
