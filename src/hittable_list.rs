use crate::hittable::*;
use crate::ray::Ray;

pub struct Hittable_List {
    list: Vec<Box<dyn Hittable>>,
}

impl Hittable_List {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Hittable_List {
        Hittable_List { list }
    }
}

impl Hittable for Hittable_List {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = None;
        let mut closest_so_far: f32 = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        return hit_anything;
    }
}
