use crate::{
    color,
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    rtweekend::{clamp, random_num},
    Vec3,
};

pub trait Material {
    /// Given a `Ray` that impacts this material and a `HitRecord`:
    /// - Return `None` if the given ray is absorbed
    /// - Return a scattered ray and its attenuation as a `Color` otherwise
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

/// Material that always scatters rays and attenuates its reflectance by its `albedo` color.
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

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Dielectric {
        Dielectric { refraction_idx }
    }

    /// Calculate the reflectance given the cosine of the angle and a
    /// refraction index
    fn reflectance(cos_theta: f64, refraction_idx: f64) -> f64 {
        // Using Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = color!(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        // I had forgotten the '-' sign here. No work since October just for this lmao
        let cos_theta = hit.normal.dot(-ray_in.direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Must reflect, ergo, cannot refract
        let must_reflect = refraction_ratio * sin_theta > 1.0;

        let reflectance_too_high =
            Dielectric::reflectance(cos_theta, refraction_ratio) > random_num();
        // let reflectance_too_high = false;

        let new_direction = if must_reflect || reflectance_too_high {
            ray_in.direction.reflect(hit.normal)
        } else {
            ray_in.direction.refract(hit.normal, refraction_ratio)
        };

        let refracted_ray = Ray::new(hit.point, new_direction);

        Some((refracted_ray, attenuation))
    }
}
