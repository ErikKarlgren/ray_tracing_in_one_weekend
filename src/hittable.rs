use std::ops::Range;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: f64,
    pub front_face: bool,
}

impl HitRecord {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
