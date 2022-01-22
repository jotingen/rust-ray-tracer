mod ray;
mod vec3;
mod hittable;
mod hittable_list;
mod sphere;

use crate::ray::*;
use crate::vec3::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::sphere::*;

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as u32,
        (255.999 * pixel_color.y()) as u32,
        (255.999 * pixel_color.z()) as u32
    );
}

fn ray_color(r: Ray, world: &mut HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    //World
    let mut world = HittableList::new();
    world.push(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0));

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
            let pixel_color: Color = ray_color(r, &mut world);

            write_color(pixel_color);
        }
    }
    eprintln!();
    eprintln!("Done");
}
