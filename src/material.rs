use crate::ray::Ray;
use crate::vec3::*;
use crate::util::Lib;
use crate::hittable::HitRecord;

#[derive(Clone,Copy)]
pub enum Material{
    Lambertian{albedo: Vec3},
    Metal{albedo: Vec3, fuzz: f32},
    Dielectric{ref_idx: f32}
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

        &Material::Metal{ albedo, fuzz } => { 
            let reflected = reflect( &Vec3::unit_vector(&r_in.direction()), &rec.normal);
            
            if fuzz < 1.0{
                *scattered = Ray::ray(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
            }
            else {
                *scattered = Ray::ray(rec.p, reflected + 1.0 * Vec3::random_in_unit_sphere());
            } 
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
        }

        &Material::Dielectric {ref_idx} =>{
            *attenuation = Color::new(1.0, 1.0, 1.0);
            let etai_over_etat: f32 ;
            if rec.front_face
            {
                etai_over_etat = 1.0/ ref_idx;
            }
            else
            {
                etai_over_etat = ref_idx;
            }

            let unit_direction: Vec3 = Vec3::unit_vector(&r_in.direction());
            
            let cos_theta: f32 = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
            let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();
            
            if etai_over_etat * sin_theta > 1.0
            {
                let reflected = reflect(&unit_direction, &rec.normal);
                *scattered = Ray::ray(rec.p, reflected);
                return true;
            }

            let reflect_prob = schlick(cos_theta, etai_over_etat);
            if Lib::random_double() < reflect_prob
            {
                let reflected = reflect(&unit_direction, &rec.normal);
                *scattered = Ray::ray(rec.p, reflected);
                return true;
            }
            
            let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
            *scattered = Ray::ray(rec.p, refracted);
            true
        }
    }
}

pub fn reflect (v:&Vec3, n: &Vec3) -> Vec3
{
    *v - 2.0 * Vec3::dot(v, n) * *n
}

pub fn refract (uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3{
    let cos_theta            = Vec3::dot(&(- *uv), n);
    let r_out_perp: Vec3     = etai_over_etat * (*uv + (cos_theta * *n));
    let r_out_parallel: Vec3 = -((1.0 - r_out_perp.squared_length()).abs().sqrt()) * *n;
    r_out_perp + r_out_parallel
}

pub fn schlick (cosine: f32, ref_idx: f32) -> f32
{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
