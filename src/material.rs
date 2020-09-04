use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::*;

#[derive(Clone,Copy)]
pub enum Material{
    Lambertian{albedo: Vec3},
    Metal{albedo: Vec3},
    Dielectric{}
}

impl Default for Material{
    fn default() -> Material{
        Material::Lambertian{albedo: Color::default()}
    }
}

pub fn scatter (material: &Material, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
    
    match material{
        &Material::Lambertian{albedo} => {
            let scatter_direction: Vec3 = rec.normal  + Vec3::random_in_unit_sphere();
            *scattered = Ray::ray(rec.p,scatter_direction);
            *attenuation = albedo;
            return true
        }

        &Material::Metal{ albedo } => {
            let reflected = reflect( &Vec3::unit_vector(&r_in.direction()), &rec.normal);
            *scattered = Ray::ray(rec.p, reflected);
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
        }

        &Material::Dielectric {} =>{false}
    }
}

pub fn reflect (v:&Vec3, n: &Vec3) -> Vec3
{
    *v - 2.0 * Vec3::dot(v, n) * *n
}
