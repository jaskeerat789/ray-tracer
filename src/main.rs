mod ray;
mod vec3;
mod util;
mod camera;
mod sphere;
mod hittable;
mod hittable_list;

use ray::Ray;
use util::Lib;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use camera::Camera;
use sphere::Sphere;
use hittable::{HitRecord,Hittable};
use hittable_list::*;

fn color(r:&Ray, world: &Hittable_List, depth: i8)->Vec3{

    let mut rec: HitRecord = HitRecord::default();

    if depth <= 0
    {
        return Color::new(0.0,0.0,0.0)
    }

    if world.hit(r,0.001,std::f32::MAX, & mut rec)
    {
        let target: Point3 = rec.p() + Vec3::random_in_hemisphere(&rec.normal());
        return 0.5 * color(&Ray::ray(rec.p(), target - rec.p()), world, depth -1);
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t:f32  = 0.5 * (unit_direction.y()+1.0);
    return (1.0 -t) * Color::new(1.0, 1.0, 1.0) + t*Color::new(0.05, 0.7, 1.0);
}


fn write_ppm (image_width:i32, image_height:i32, max_value:i16){

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0),0.5)));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0)));
    let world = Hittable_List::new(list);

    let sample_per_pixel:i16 = 100;
    let max_depth:i8 = 50; 
    let cam: Camera = Camera::new();

    println!("P3\n{} {}\n{}",image_width,image_height,max_value);

    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample_per_pixel{
                let u:f32 = (i as f32 + Lib::random_double() ) / (image_width - 1) as f32;
                let v:f32 = (j as f32 + Lib::random_double() ) / (image_height - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
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
