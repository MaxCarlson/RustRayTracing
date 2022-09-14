mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use std::io::{stderr, Write};
use rand::{thread_rng, Rng};

use vec::{Vec3, Point3, Color, FloatT};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;

fn ray_color(r: &Ray, world: &World) -> Color {

    if let Some(rec) = world.hit(r, 0.0, FloatT::INFINITY)  {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_dir = r.direction().normalized();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

// https://misterdanb.github.io/raytracinginrust/#outputanimage/theppmimageformat
fn main()
{
    // Image
    const ASPECT_RATIO: FloatT = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as FloatT) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));


    // Camera
    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev()
    {
        eprint!("\rScanlines remaining {:3}", IMAGE_HEIGHT - j - 1);
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
                pixel_color += ray_color(&r, &world);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done");
}
