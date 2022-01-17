fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3");
    println!("{} {} {}", image_width, image_height, 255);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width - 1) as f32;
            let g: f32 = j as f32 / (image_height - 1) as f32;
            let b: f32 = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
