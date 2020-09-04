mod ray;
mod vec3;
mod util;
mod camera;
mod sphere;
mod hittable;
mod material;
mod hittable_list;

use ray::Ray;
use util::Lib;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use camera::Camera;
use sphere::Sphere;
use material::*;
use hittable::{HitRecord,Hittable};
use hittable_list::*;

fn color(r:&Ray, world: &Hittable_List, depth: i8)->Vec3{
    if depth <= 0
    {
        return Color::new(0.0,0.0,0.0)
    }

    if let Some(rec) = world.hit(r,0.001,std::f32::MAX)
    {
        let mut scattered: Ray = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation: Color = Color::default();
        if scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, world, depth-1);
        }
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t:f32  = 0.5 * (unit_direction.y()+1.0);
    return  (1.0 -t) * Color::new(1.0, 1.0, 1.0) + t*Color::new(0.05, 0.7, 1.0);
}


fn write_ppm (image_width:i32, image_height:i32, max_value:i16){

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Material::Lambertian{ albedo: Color::new(0.8,0.8,0.0) };
    let material_center = Material::Lambertian{ albedo: Color::new(0.7,0.3,0.3) };
    let material_left   = Material::Metal{ albedo: Color::new(0.99,0.99,0.99) };
    let material_right  = Material::Metal{ albedo: Color::new(0.8,0.6,0.2) };

    list.push(Box::new(Sphere::sphere(Vec3::new( 0.0, -100.5, -1.0 ), 100.0, material_ground )));
    list.push(Box::new(Sphere::sphere(Vec3::new( 0.0,    0.0, -1.0 ),   0.5, material_center )));
    list.push(Box::new(Sphere::sphere(Vec3::new(-1.0,    0.0, -1.0 ),   0.5, material_left   )));
    list.push(Box::new(Sphere::sphere(Vec3::new( 1.0,    0.0, -1.0 ),   0.5, material_right  )));

    let world = Hittable_List::new(list);

    let sample_per_pixel:i16 = 100;
    let max_depth:i8 = 50; 
    let cam: Camera = Camera::new();

    println!("P3\n{} {}\n{}",image_width,image_height,max_value);

    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample_per_pixel{
                let u: f32  = (i as f32 + Lib::random_double() ) / (image_width - 1) as f32;
                let v: f32  = (j as f32 + Lib::random_double() ) / (image_height - 1) as f32;
                let r: Ray  = cam.get_ray(u, v);
                pixel_color = pixel_color + color(&r,&world,max_depth)
            }

            pixel_color = pixel_color / sample_per_pixel as f32;

            let ir:i32 = (255.99 * ( pixel_color.x()).sqrt() ) as i32;
            let ig:i32 = (255.99 * ( pixel_color.y()).sqrt() ) as i32;
            let ib:i32 = (255.99 * ( pixel_color.z()).sqrt() ) as i32;
            println!("{} {} {}",ir,ig,ib);

        }
    }
}

fn main() {

    const ASPECT_RATIO:f32 = 16.0/9.0;
    const MAX_VALUE:i16 = 255;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    write_ppm(IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE)

}
