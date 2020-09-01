mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;

use ray::Ray;
use vec3::Vec3;
use vec3::Point3;
use vec3::Color;
use sphere::Sphere;
use hittable::{HitRecord,Hittable};
use hittable_list::*;


fn color(r:&Ray, world: &Hittable_List)->Vec3{

    let mut rec: HitRecord = HitRecord::default();

    if (world.hit(r,0.0,std::f32::MAX, & mut rec))
    {
        return 0.5 * (Vec3::new(
            rec.normal().x(),
            rec.normal().y(),
            rec.normal().z()
        ) + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t:f32  = 0.5 * (unit_direction.y()+1.0);
    return (1.0 -t) * Color::new(1.0, 1.0, 1.0) + t*Color::new(0.05, 0.7, 1.0);
}

fn hit_sphere (center: &Point3, radius: f32, r: &Ray) -> f32{

    let oc: Vec3 = r.origin() - *center;
    let a: f32 = r.direction().squared_length();
    let half_b: f32 = Vec3::dot(&oc, &r.direction());
    let c: f32 = oc.squared_length() - radius*radius;
    let discriminant = half_b * half_b - a * c;
    
    if discriminant < 0.0
    {
        -1.0
    }
    else
    {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn write_sample_ppm (image_width:i32,image_height:i32,max_value:i32){
    println!("P3\n{} {}\n{}",image_width,image_height,max_value);
    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let r:f32 = i as f32 / image_width as f32;
            let g:f32 = j as f32 / image_height as f32;
            let b:f32 = 0.5;

            let ir:i32 = (255.99 * r) as i32;
            let ig:i32 = (255.99 * g) as i32;
            let ib:i32 = (255.99 * b) as i32;

            println!("{} {} {}",ir,ig,ib);

        }
    }

}

fn write_ppm (aspect_ratio:f32, image_width:i32, image_height:i32, max_value:i16){

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0),0.5)));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0)));
    let world = Hittable_List::new(list);


    let viewport_height:f32 = 2.0;
    let viewport_width:f32 = aspect_ratio * viewport_height;
    let focal_length:f32 = 1.0;

    let origin : Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal : Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical : Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner : Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n{}",image_width,image_height,max_value);

    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let u:f32 = i as f32 / (image_width - 1) as f32;
            let v:f32 = j as f32 / (image_height - 1) as f32;
            
            let r: Ray = Ray::ray(origin, lower_left_corner+ horizontal*u + vertical*v + -origin) ;
            let col: Vec3 = color(&r,&world);

            let ir:i32 = (255.99 * col.x()) as i32;
            let ig:i32 = (255.99 * col.y()) as i32;
            let ib:i32 = (255.99 * col.z()) as i32;
            println!("{} {} {}",ir,ig,ib);

        }
    }
}

fn main() {

    // Sample PPM file generation,
    
    // let image_width:i32 = 400;
    // let image_height:i32 = 200;
    // let max_value:i32 = 255;
    //write_sample_ppm(image_width, image_height, max_value);

    const ASPECT_RATIO:f32 = 16.0/9.0;
    const MAX_VALUE:i16 = 255;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    write_ppm(ASPECT_RATIO, IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE)

}
