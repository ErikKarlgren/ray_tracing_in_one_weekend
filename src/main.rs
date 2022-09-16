mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use std::fmt::Write;
use vec3::Vec3;

/// Returns the background color
fn ray_color(ray: Ray) -> Color {
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
    eprintln!("\nDone!");
}

fn write_color(image: &mut String, color: Color) {
    let Color { red, green, blue } = color;
    let red = (255.999 * red) as i32;
    let green = (255.999 * green) as i32;
    let blue = (255.999 * blue) as i32;
    writeln!(image, "{red} {green} {blue}").unwrap();
}
