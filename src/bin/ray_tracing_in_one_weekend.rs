use ray_tracing_in_one_weekend::{create_image, HittableList, Sphere, Vec3};

fn main() {
    let world = create_world();
    let image = create_image(&world);
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
