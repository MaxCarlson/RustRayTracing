mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use vec::{Vec3, Point3, Color, FloatT};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

use std::io::{stderr, Write};
use rand::{Rng, thread_rng};
use std::rc::Rc;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    // Maximum ray-bounce depth has been reached
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, FloatT::INFINITY)  {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::default()
        }
    } else {
        let unit_dir = r.direction().normalized();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    } 

    // Two scattering methods       
    // Lambertian scattering
    // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
    // Hemisphere scattering
    // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
}

// https://misterdanb.github.io/raytracinginrust/#outputanimage/theppmimageformat
fn main() {
    // Image
    const ASPECT_RATIO: FloatT = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as FloatT) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 8;

    // World
    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_left_inner = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_left_inner));
    world.push(Box::new(sphere_right));

    // Camera
    let cam = Camera::new(Point3::new(-2.0, 2.0, 1.0),
                        Point3::new(0.0, 0.0, -1.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        30.0,
                        ASPECT_RATIO);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut count = 0;
    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev()
    {
        eprint!("\r {:3}/{} scanlines", IMAGE_HEIGHT - j - 1, IMAGE_HEIGHT);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH
        {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: FloatT = rng.gen();
                let random_v: FloatT = rng.gen();

                let u = ((i as FloatT) + random_u) / ((IMAGE_WIDTH - 1) as FloatT);
                let v = ((j as FloatT) + random_v) / ((IMAGE_HEIGHT - 1) as FloatT);
    
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
            count += 1;
        }
    }
    eprintln!("Done");
}
