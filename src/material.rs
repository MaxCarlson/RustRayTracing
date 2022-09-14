use super::hit::{HitRecord};
use super::ray::{Ray};
use super::vec::{Vec3, Color, FloatT};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}


pub struct Metal {
    albedo: Color,
    fuzz: FloatT
}

pub struct Lambertian {
    albedo: Color
}

pub struct Dielectric {
    ir: FloatT
}

impl Metal {
    pub fn new(a: Color, f: FloatT) -> Metal {
        Metal {
            albedo: a,
            fuzz: f
        }
    }
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: a
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: FloatT) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();

        // If our random point on unit sphere is equal to our normal 
        // it's possible scatter_direction is NaN/INF - this prevents that degenerate case
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
    
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r_in.direction().normalized();
        let cos_theta = ((-1.0) * unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            unit_dir.reflect(rec.normal)
        } else {
            unit_dir.refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}


