use crate::hittable::*;
use crate::ray::Ray;

pub struct Hittable_List{
    list: Vec<Box<dyn Hittable>>,
}

impl Hittable_List{
    pub fn new(list:Vec<Box<dyn Hittable>>) -> Hittable_List{
        Hittable_List{list}
    } 
}

impl Hittable for Hittable_List{
    fn hit(&self,r:&Ray, t_min:f32, t_max:f32, rec: &mut HitRecord) -> bool{
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;

        for object in &self.list{
            if object.hit(r,t_min,closest_so_far,&mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_t(closest_so_far);
                rec.set_p(temp_rec.p());
                rec.set_normal(temp_rec.normal());
            }
        }
        return hit_anything;
    }
}
