mod color;
mod ray;
mod vec3;
mod hittable;
mod sphere;

use color::Color;
use ray::Ray;
use std::fmt::Write;
use vec3::Vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0; // width / height
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // Estimated chars per color in image: 12
    // 50 extra bytes for header, just in case
    let mut image = String::with_capacity(image_width * image_height * 12 + 50);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    write!(&mut image, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for y in (0..image_height).into_iter().rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(ray);
            write_color(&mut image, color);
        }
    }
    print!("{image}");
}

/// Returns the background color
fn ray_color(ray: Ray) -> Color {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    if let Some(dist) = hit_sphere(sphere_center, 0.5, ray) {
        let normal_vec = ray.at(dist) - sphere_center;
        return 0.5 * Color::new(normal_vec.x + 1.0, normal_vec.y + 1.0, normal_vec.z + 1.0);
    }

    // We generate a white-blue gradient based on the 'y' coordinate.
    // The higher the 'y', the 'bluer' the pixel.
    // The lower the 'y', the whiter the pixel.
    // Since Rays use unit vectors for directions, it will be blue
    // the most on the middle-top of the image
    let t = 0.5 * (ray.direction.y + 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    // This is called a 'linear interpolation'
    (1.0 - t) * white + t * blue
}

/// If the ray hits the sphere, return the distance from the
/// ray's origin to the nearest hit point to the sphere.
fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> Option<f64> {
    // We'll use the quadratic formula to check if a ray hits a sphere
    // at² + bt + c = 0
    // 't' is the distance from the ray's origin to where it hits the sphere
    let oc = ray.origin - center;

    let a = ray.direction.length_squared();
    // let b = (2.0 * ray.direction).dot(oc);
    let half_b = ray.direction.dot(oc);
    let c = oc.length_squared() - (radius * radius);

    // t = (-b +- sqrt(b² - 4*a*c)/2*a)
    let discriminant = half_b * half_b - a * c;

    // discriminant > 0   => 2 real solutions  => ray hits sphere
    // discriminant == 0  => 1 real solution   => ray is tangent to sphere
    // discriminant < 0   => no real solutions => ray does not hit sphere
    if discriminant < 0.0 {
        None
    } else {
        // It seems we only care about the hit point we can actually see,
        // so we just calculate one and not both.
        // If there was only one hit point, we get it anyways.
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn write_color(image: &mut String, color: Color) {
    let Color { red, green, blue } = color;
    let red = (255.999 * red) as i32;
    let green = (255.999 * green) as i32;
    let blue = (255.999 * blue) as i32;
    writeln!(image, "{red} {green} {blue}").unwrap();
}
