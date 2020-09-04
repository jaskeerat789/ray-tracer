use crate::hittable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn sphere(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().squared_length();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = oc.squared_length() - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root: f32 = discriminant.sqrt();

            let mut temp: f32 = (-half_b - root) / a;
            if (temp < t_max) && (temp > t_min) {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }

            temp = (-half_b + root) / a;
            if (temp < t_max) && (temp > t_min) {
                return Some(HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }
        }
        None
    }
}
