use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal (&mut self, r: &Ray, outward_normal: &Vec3)
    {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        if self.front_face
        {
            self.normal =   *outward_normal;
        }
        else {
            self.normal = - *outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }
}
