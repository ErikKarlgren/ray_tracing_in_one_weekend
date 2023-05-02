mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vector3;

pub use {
    camera::Camera,
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    rtweekend::{random_num, random_num_in_range},
    sphere::Sphere,
    vector3::Vec3,
};

use {ray::Ray, rtweekend::clamp};

use std::fmt::Write;

pub fn create_image(
    world: &HittableList,
    camera: &Camera,
    image_width: usize,
    samples_per_pixel: u32,
) -> String {
    // Image
    let image_height: usize = (image_width as f64 / camera.aspect_ratio()) as usize;
    let max_depth = 50;

    let mut image = String::new();

    // Render
    write!(&mut image, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut pixel_color = color!(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((x as f64) + random_num()) / ((image_width - 1) as f64);
                let v = ((y as f64) + random_num()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, world, max_depth);
            }
            write_color(&mut image, pixel_color, samples_per_pixel);
        }
    }
    image
}

/// Returns the background color
fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return color!(0.0, 0.0, 0.0);
    }

    let desired_hit_distance = 0.001..f64::INFINITY;

    if let Some(hit) = world.hit(ray, &desired_hit_distance) {
        if let Some((scattered_ray, attenuation)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered_ray, world, depth - 1);
        }
        return color!(0.0, 0.0, 0.0);
    }

    // We generate a white-blue gradient based on the 'y' coordinate.
    // The higher the 'y', the 'bluer' the pixel.
    // The lower the 'y', the whiter the pixel.
    // Since Rays use unit vectors for directions, it will be blue
    // the most on the middle-top of the image
    let t = 0.5 * (ray.direction.y + 1.0);
    let white = color!(1.0, 1.0, 1.0);
    let blue = color!(0.5, 0.7, 1.0);

    // This is called a 'linear interpolation'
    (1.0 - t) * white + t * blue
}

fn write_color(image: &mut String, color: Color, samples_per_pixel: u32) {
    let Color {
        mut red,
        mut green,
        mut blue,
    } = color;

    // Divide color by number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let scale_color = |c: f64| (c * scale).sqrt();

    red = scale_color(red);
    green = scale_color(green);
    blue = scale_color(blue);

    let translate_color = |c| 256.0 * clamp(c, 0.0..=0.999);

    let red = translate_color(red) as i32;
    let green = translate_color(green) as i32;
    let blue = translate_color(blue) as i32;

    writeln!(image, "{red} {green} {blue}").unwrap();
}
