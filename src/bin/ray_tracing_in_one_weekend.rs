use std::rc::Rc;

use ray_tracing_in_one_weekend::{
    color, create_image, random_num, random_num_in_range, vec3, Camera, Color, Dielectric,
    HittableList, Lambertian, Metal, Sphere, Vec3,
};

fn main() {
    let world = random_world();
    let camera = Camera::builder()
        .look_from(vec3!(13.0, 2.0, 3.0))
        .look_at(Vec3::zero())
        .vertical_fov(20.0.into())
        .aspect_ratio(3.0 / 2.0)
        .up_vector(vec3!(0.0, 1.0, 0.0))
        .focus_distance(10.0)
        .aperture(0.1)
        .build();

    let image = create_image(&world, &camera, 600, 100);
    print!("{image}");
}

/// Creates a `HittableList` pre-populated with several items
fn random_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(color!(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        vec3!(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let (a, b) = (a as f64, b as f64);
            let choose_mat = random_num();
            let center = vec3!(a + 0.9 * random_num(), 0.2, b + 0.9 * random_num());

            if (center - vec3!(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                // diffuse
                let albedo = random_color() * random_color();
                let material = Lambertian::new(albedo);
                world.add(Sphere::new(center, 0.2, Rc::new(material)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_color_in_range(0.5..1.0);
                let fuzz = random_num_in_range(0.0..0.5);
                let material = Metal::new(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, Rc::new(material)));
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere::new(center, 0.2, Rc::new(material)));
            };
        }
    }

    [
        Sphere::new(vec3!(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5))),
        Sphere::new(
            vec3!(-4.0, 1.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(color!(0.4, 0.2, 0.1))),
        ),
        Sphere::new(
            vec3!(4.0, 1.0, 0.0),
            1.0,
            Rc::new(Metal::new(color!(0.7, 0.6, 0.5), 0.0)),
        ),
    ]
    .into_iter()
    .for_each(|sphere| world.add(sphere));

    world
}

fn random_color() -> Color {
    color!(random_num(), random_num(), random_num())
}

fn random_color_in_range(range: std::ops::Range<f64>) -> Color {
    color!(
        random_num_in_range(range.clone()),
        random_num_in_range(range.clone()),
        random_num_in_range(range)
    )
}
