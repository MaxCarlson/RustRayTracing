mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;
mod light; 

use vec::{Vec3, Point3, Color, FloatT};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use light::{Light, Lights};

use std::io::{stderr, Write};
use rand::{Rng, thread_rng, random};
use std::sync::Arc;
use rayon::prelude::*;
use std::fs::File;


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
        Color::default()
        //let unit_dir = r.direction().normalized();
        //let t = 0.5 * (unit_dir.y() + 1.0);
        //(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    } 

    // Two scattering methods       
    // Lambertian scattering
    // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
    // Hemisphere scattering
    // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
}

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                // let sphere_mat = Arc::new(Dielectric::new(1.5));
                // Dielectrics
                let sphere_mat = Arc::new(Dielectric::new(rng.gen_range(0.01..20.0)));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn debug_scene() -> World {
    let mut world = World::new();
    let mut lights = Lights::new(); 

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_left_inner = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    let light1 = Light::new(Point3::new(0.0, 15.0, 0.0), Color::new(1.0, 1.0, 1.0), Vec3::new(0.0, -1.0, 0.0));

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_left_inner));
    world.push(Box::new(sphere_right));   

    world
}

// https://misterdanb.github.io/raytracinginrust/#outputanimage/theppmimageformat
fn main() {
    
    // Image
    const ASPECT_RATIO: FloatT = 16.0 / 9.0;
    let mut IMAGE_WIDTH: u64 = 1200;
    let mut IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as FloatT) / ASPECT_RATIO) as u64;
    let mut SAMPLES_PER_PIXEL: u64 = 32;
    let mut MAX_DEPTH: u64 = 16;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Mode {
        FAST_DEBUG,
        DEBUG,
        RUN
    }

    let mut world = World::new();    
    //const runMode: Mode = Mode::DEBUG;
    //const runMode: Mode = Mode::FAST_DEBUG;
    const runMode: Mode = Mode::DEBUG;


    if runMode == Mode::FAST_DEBUG {
        world = debug_scene();
        IMAGE_WIDTH = 1200;
        IMAGE_HEIGHT = ((IMAGE_WIDTH as FloatT) / ASPECT_RATIO) as u64;
        SAMPLES_PER_PIXEL = 8;
        MAX_DEPTH = 8;
    } else if runMode == Mode::DEBUG {
        world = random_scene();
        IMAGE_WIDTH = 1200;
        IMAGE_HEIGHT = ((IMAGE_WIDTH as FloatT) / ASPECT_RATIO) as u64;
        SAMPLES_PER_PIXEL = 32;
        MAX_DEPTH = 16;
    } else {
        world = random_scene();
    }

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let fov = 20.0;

    let cam = Camera::new(lookfrom,
                          lookat,
                          vup,
                          fov,
                          ASPECT_RATIO,
                          aperture,
                          dist_to_focus);

    let filename = "./image.ppm";
    let mut buffer = File::create(filename).unwrap();
    
    writeln!(&mut buffer, "P3");
    writeln!(&mut buffer, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    writeln!(&mut buffer, "255");


    //let mut count = 0;
    for j in (0..IMAGE_HEIGHT).rev()
    {
        eprint!("\r {:3}/{} scanlines", IMAGE_HEIGHT - j - 1, IMAGE_HEIGHT);
        stderr().flush().unwrap();

        let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let mut rng = thread_rng();
                let random_u: FloatT = rng.gen();
                let random_v: FloatT = rng.gen();

                let u = ((i as FloatT) + random_u) / ((IMAGE_WIDTH - 1) as FloatT);
                let v = ((j as FloatT) + random_v) / ((IMAGE_HEIGHT - 1) as FloatT);
    
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color
        }).collect();

        for pixel_color in scanline {
            writeln!(&mut buffer, "{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done");
}

//// Python
//def main():
//    print("Hello World")
//
//// C++
//#include <iostream>
//void main()
//{
//    stdd::cout << "Hello World" << std::endl;
//}
//
////Rust
//fn main() {
//    eprintln!("Hello World");
//}