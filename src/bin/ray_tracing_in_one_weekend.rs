use ray_tracing_in_one_weekend::{
    clamp, random_num, Camera, Color, Hittable, HittableList, Ray, Sphere, Vec3,
};

use std::fmt::Write;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0; // width / height
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    let mut image = String::new();

    // World
    let world = create_world();

    // Camera
    let camera = Camera::new(aspect_ratio);

    // Random numbers
    // let mut random = thread_rng();
    // let distr = Uniform::from(0.0..1e7);

    // Render
    write!(&mut image, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for y in (0..image_height).into_iter().rev() {
        for x in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((x as f64) + random_num()) / ((image_width - 1) as f64);
                let v = ((y as f64) + random_num()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            write_color(&mut image, pixel_color, samples_per_pixel);
        }
    }
    print!("{image}");
}

/// Creates a `HittableList` pre-populated with several items
fn create_world() -> HittableList {
    let mut world = HittableList::new();
    [
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]
    .into_iter()
    .for_each(|h| world.add(h));
    world
}

/// Returns the background color
fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(ray, &(0.0..f64::INFINITY)) {
        return 0.5 * Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
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

fn write_color(image: &mut String, color: Color, samples_per_pixel: u32) {
    let Color {
        mut red,
        mut green,
        mut blue,
    } = color;

    // Divide color by number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    red *= scale;
    green *= scale;
    blue *= scale;

    let translate_color = |c| 256.0 * clamp(c, 0.0..=0.999);

    let red = translate_color(red) as i32;
    let green = translate_color(green) as i32;
    let blue = translate_color(blue) as i32;

    writeln!(image, "{red} {green} {blue}").unwrap();
}
