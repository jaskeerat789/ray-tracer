use crate::ray::*;
use crate::util::Lib;
use crate::vec3::*;
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let off_set: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray::ray(
            self.origin + off_set,
            self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - off_set,
        )
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new(
            Lib::random_min_max(-1.0, 1.0),
            Lib::random_min_max(-1.0, 1.0),
            0.0,
        );
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
