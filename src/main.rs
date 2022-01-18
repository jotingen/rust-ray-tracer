mod Vec3;
mod Ray;

use crate::Vec3::Color;

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u32,
        (255.999 * pixel_color.y()) as u32,
        (255.999 * pixel_color.z()) as u32
    );
}

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3");
    println!("{} {} {}", image_width, image_height, 255);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color: Color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            write_color(pixel_color);
        }
    }
    eprintln!();
    eprintln!("Done");
}
