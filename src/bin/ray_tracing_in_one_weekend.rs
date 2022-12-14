use std::rc::Rc;

use ray_tracing_in_one_weekend::{
    create_image, Color, HittableList, Lambertian, Metal, Sphere, Vec3,
};

fn main() {
    let world = create_world();
    let image = create_image(&world, 400);
    print!("{image}");
}

/// Creates a `HittableList` pre-populated with several items
fn create_world() -> HittableList {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let left_material = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let right_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = HittableList::new();
    [
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material),
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, center_material),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left_material),
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right_material),
    ]
    .into_iter()
    .for_each(|h| world.add(h));
    world
}
