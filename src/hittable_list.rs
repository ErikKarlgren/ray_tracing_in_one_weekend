use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::ops::Range;

/// List of items that implement the trait `Hittable`.
/// This list too implements that trait.
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Creates an empty `HittableList`
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    /// Adds a `Hittable` item to this list
    pub fn add<H>(&mut self, hittable: H)
    where
        H: Hittable + 'static,
    {
        self.objects.push(Box::new(hittable));
    }

    /// Clears this list, removing all items
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, hit_range: &Range<f64>) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_hit_dist = hit_range.end;

        for obj in &self.objects {
            let new_range = hit_range.start..closest_hit_dist;
            if let opt_hit @ Some(hit) = obj.hit(ray, &new_range) {
                closest_hit = opt_hit;
                closest_hit_dist = hit.dist;
            }
        }
        closest_hit
    }
}
