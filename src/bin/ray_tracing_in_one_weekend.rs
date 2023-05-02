use std::rc::Rc;

use ray_tracing_in_one_weekend::{
    create_image, vec3, Camera, Color, Dielectric, HittableList, Lambertian, Metal, Sphere, Vec3,
};

fn main() {
    let world = create_world();
    let camera = Camera::builder()
        .look_from(vec3!(-2.0, 2.0, 1.0))
        .look_at(vec3!(0.0, 0.0, -1.0))
        .vertical_fov(20.0.into())
        .aspect_ratio(16.0 / 9.0)
        .build();
    let image = create_image(&world, &camera, 400);
    print!("{image}");
}

/// Creates a `HittableList` pre-populated with several items
fn create_world() -> HittableList {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_material = Rc::new(Dielectric::new(1.5));
    let right_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    [
        Sphere::new(vec3!(0.0, -100.5, -1.0), 100.0, ground_material),
        Sphere::new(vec3!(0.0, 0.0, -1.0), 0.5, center_material),
        Sphere::new(vec3!(-1.0, 0.0, -1.0), 0.5, left_material.clone()),
        Sphere::new(vec3!(-1.0, 0.0, -1.0), -0.45, left_material),
        Sphere::new(vec3!(1.0, 0.0, -1.0), 0.5, right_material),
    ]
    .into_iter()
    .for_each(|h| world.add(h));
    world
}
