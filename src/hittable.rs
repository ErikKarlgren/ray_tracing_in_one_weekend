use std::ops::Range;

use crate::{ray::Ray, vec3::Vec3};

/// Struct that contains all the data about a `Ray` hitting
/// a 3D structure
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Create a new `HitRecord`
    pub fn new(point: Vec3, normal: Vec3, dist: f64, ray: &Ray) -> HitRecord {
        let front_face = ray.direction.dot(normal) < 0.0;
        HitRecord {
            point,
            normal: if front_face { normal } else { -normal },
            dist,
            front_face,
        }
    }
}

/// Trait for 3D structures that can be hitten by a `Ray`
pub trait Hittable {
    /// Return a `HitRecord` if a given `Ray` hits this structure at a distance
    /// from the ray's origin that is in a given hit range.
    fn hit(&self, ray: &Ray, hit_range: &Range<f64>) -> Option<HitRecord>;
}
