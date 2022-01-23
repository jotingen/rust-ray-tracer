mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

use rand::{thread_rng, Rng};

fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    //Write out the translated [0,255] value of each color component
    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32
    );
}

fn ray_color(r: Ray, world: &mut HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

fn main() {
    let mut rng = thread_rng();

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;

    //World
    let mut world = HittableList::new();
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    //Camera
    let cam: Camera = Camera::new();

    //Render
    println!("P3");
    println!("{} {} {}", image_width, image_height, 255);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height as f64 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(r, &mut world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!();
    eprintln!("Done");
}
