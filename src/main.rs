mod ray;
mod vec3;
mod util;
mod sphere;
mod camera;
mod hittable;
mod material;
mod hittable_list;

use ray::Ray;
use util::Lib;
use material::*;
use sphere::Sphere;
use camera::Camera;
use hittable_list::*;
use hittable::Hittable;
use vec3::{Vec3,Point3,Color};

fn color(r: &Ray, world: &HittableList, depth: i8) -> Vec3 {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        let mut scattered: Ray = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation: Color = Color::default();
        if scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth - 1);
        }
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.05, 0.7, 1.0);
}

fn write_ppm(image_width: i32, image_height: i32, max_value: i16, aspect_ratio: f32) {

    let sample_per_pixel: i16 = 500;
    let max_depth: i8 = 50;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus: f32 = 10.0;

    let world = random_scene();
    let cam: Camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n{}", image_width, image_height, max_value);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample_per_pixel {
                let u: f32 = (i as f32 + Lib::random_double()) / (image_width - 1) as f32;
                let v: f32 = (j as f32 + Lib::random_double()) / (image_height - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color = pixel_color + color(&r, &world, max_depth)
            }

            pixel_color = pixel_color / sample_per_pixel as f32;

            let ir: i32 = (255.99 * (pixel_color.x()).sqrt()) as i32;
            let ig: i32 = (255.99 * (pixel_color.y()).sqrt()) as i32;
            let ib: i32 = (255.99 * (pixel_color.z()).sqrt()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn random_scene() -> HittableList {
    let mut objects_in_world: Vec<Box<dyn Hittable>> = Vec::new();
    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    objects_in_world.push(
        Box::new(Sphere::sphere(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )),
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = Lib::random_double();
            let center: Point3 = Point3::new(
                a as f32 + 0.9 * Lib::random_double(),
                0.2,
                b as f32 + 0.9 * Lib::random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian { albedo };
                    objects_in_world.push(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo: Color = Color::random_min_max(0.5, 1.0);
                    let fuzz: f32 = Lib::random_double();
                    let sphere_material = Material::Metal { albedo, fuzz };
                    objects_in_world.push(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric { ref_idx: 1.5 };
                    objects_in_world.push(Box::new(Sphere::sphere(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric { ref_idx: 1.5 };
    let material2 = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    objects_in_world.push(Box::new(Sphere::sphere(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    objects_in_world.push(Box::new(Sphere::sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    objects_in_world.push(Box::new(Sphere::sphere(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));
    HittableList::new(objects_in_world)
}

fn main() {
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const MAX_VALUE: i16 = 255;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE, ASPECT_RATIO)
}
