use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: &Range<f64>) -> Option<HitRecord> {
        // We'll use the quadratic formula to check if a ray hits a sphere
        // at² + bt + c = 0
        // 't' is the distance from the ray's origin to where it hits the sphere
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        // t = (-b +- sqrt(b² - 4*a*c)/2*a)
        let discriminant = half_b * half_b - a * c;

        // discriminant > 0   => 2 real solutions  => ray hits sphere
        // discriminant == 0  => 1 real solution   => ray is tangent to sphere
        // discriminant < 0   => no real solutions => ray does not hit sphere
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let dist = {
            let neg_t = (-half_b - sqrt_d) / a;
            let pos_t = (-half_b + sqrt_d) / a;
            if t_range.contains(&neg_t) {
                neg_t
            } else if t_range.contains(&pos_t) {
                pos_t
            } else {
                return None;
            }
        };

        let hit_point = ray.at(dist);
        Some(HitRecord::new(
            hit_point,
            (hit_point - self.center) / self.radius,
            dist,
        ))
    }
}
