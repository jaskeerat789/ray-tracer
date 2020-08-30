fn write_ppm (image_width:i32,image_height:i32,max_value:i32){
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

fn main() {

    let image_width:i32 = 400;
    let image_height:i32 = 200;
    let max_value:i32 = 255;
    write_ppm(image_width, image_height, max_value);

}
