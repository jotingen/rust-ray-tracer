mod ray;
mod vec3;

use crate::ray::*;
use crate::vec3::*;

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u32,
        (255.999 * pixel_color.y()) as u32,
        (255.999 * pixel_color.z()) as u32
    );
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    }
    ( -half_b - discriminant.sqrt() ) / a
}
fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r); 
    if t > 0.0 {
        let N = (r.at(t) - Vec3::new(0.0,0.0,-1.0)).unit_vector();
        return 0.5*Color::new(N.x()+1.0, N.y()+1.0, N.z()+1.0)
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    //Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    println!("P3");
    println!("{} {} {}", image_width, image_height, 255);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u: f64 = i as f64 / (image_width as f64 - 1.0);
            let v: f64 = j as f64 / (image_height as f64 - 1.0);
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color: Color = ray_color(r);

            write_color(pixel_color);
        }
    }
    eprintln!();
    eprintln!("Done");
}
