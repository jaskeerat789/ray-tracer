use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere{
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn sphere (center: Point3, radius: f32) -> Sphere{
        Sphere{center, radius}
    }
}

impl Hittable for Sphere{
    
    fn hit(&self, r:&Ray, t_min:f32, t_max:f32, rec: &mut HitRecord) -> bool{
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().squared_length();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = oc.squared_length() - self.radius * self.radius;
        let discriminant: f32 = half_b*half_b - a*c;
        if discriminant > 0.0 {
            let root: f32 = discriminant.sqrt();

            let mut temp: f32 = (-half_b - root) / a;
            if (temp < t_max ) && (temp> t_min){
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }

            temp = (-half_b + root) / a;
            if(temp < t_max ) && (temp> t_min){
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
            
        }
        return false;
    }
}