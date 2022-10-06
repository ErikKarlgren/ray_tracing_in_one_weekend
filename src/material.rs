use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend::clamp, Vec3};

pub trait Material {
    /// Given a `Ray` that impacts this material and a `HitRecord`:
    /// - Return `None` if the given ray is absorbed
    /// - Return a scattered ray and its attenuation as a `Color` otherwise
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

/// Material that always scatters rays and attenuates its reflectance by its `albedo` color.:w
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vec();

        // To avoid a zero scatter direction vector, since it could cause
        // Infinity and NaN related problems
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered_ray = Ray::new(hit.point, scatter_direction);
        Some((scattered_ray, self.albedo.clone()))
    }
}

pub struct Metal {
    albedo: Color,
    fuzzyness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzzyness: f64) -> Metal {
        Metal {
            albedo,
            fuzzyness: clamp(fuzzyness, 0.0..=1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflection_vec = ray_in.direction.unit_vec().reflect(hit.normal);
        let scattered_ray = Ray::new(
            hit.point,
            reflection_vec + self.fuzzyness * Vec3::random_unit_vec(),
        );

        if scattered_ray.direction.dot(hit.normal) > 0.0 {
            Some((scattered_ray, self.albedo.clone()))
        } else {
            None
        }
    }
}
