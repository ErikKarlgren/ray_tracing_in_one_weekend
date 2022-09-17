use std::ops::Range;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: f64,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, dist: f64) -> HitRecord {
        HitRecord {
            point,
            normal,
            dist,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
