mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn color(r:&Ray)->Vec3{
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t:f32 = 0.5 * (unit_direction.y() + 1.0);
    (Vec3::new(1f32, 1f32, 1f32) * (1.0-t) ) + (Vec3::new(0.5f32, 0.7f32, 1.0f32) * t)
}

fn write_sample_ppm (image_width:i32,image_height:i32,max_value:i32){
    println!("P3\n{} {}\n{}",image_width,image_height,max_value);
    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let r:f32 = i as f32 / image_width as f32;
            let g:f32 = j as f32 / image_height as f32;
            let b:f32 = 0.25;

            let ir:i32 = (255.99 * r) as i32;
            let ig:i32 = (255.99 * g) as i32;
            let ib:i32 = (255.99 * b) as i32;

            println!("{} {} {}",ir,ig,ib);

        }
    }

}

fn write_ppm (){}

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

    let viewport_height:f32 = 2.0;
    let viewport_width:f32 = ASPECT_RATIO * viewport_height;
    let focal_length:f32 = 1.0;

    let origin : Vec3 = Vec3::new(0f32, 0f32, 0f32);
    let horizontal : Vec3 = Vec3::new(viewport_width, 0f32, 0f32);
    let vertical : Vec3 = Vec3::new(0f32, viewport_height, 0f32);
    let lower_left_corner : Vec3 =origin + -horizontal + -vertical + -Vec3::new(0f32, 0f32, focal_length);

    println!("P3\n{} {}\n{}",IMAGE_WIDTH,IMAGE_HEIGHT,MAX_VALUE);

    for j in (0..IMAGE_HEIGHT).rev(){
        for i in 0..IMAGE_WIDTH {
            let u:f32 = i as f32 / IMAGE_WIDTH as f32;
            let v:f32 = j as f32 / IMAGE_HEIGHT as f32;
            
            let r: Ray = Ray::ray(origin, lower_left_corner+ horizontal*u + vertical*v + -origin) ;
            let col: Vec3 = color(&r);

            let ir:i32 = (255.99 * col.x()) as i32;
            let ig:i32 = (255.99 * col.y()) as i32;
            let ib:i32 = (255.99 * col.z()) as i32;
            println!("{} {} {}",ir,ig,ib);

        }
    }
}
