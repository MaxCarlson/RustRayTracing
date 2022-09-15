use super::vec::{Vec3, Point3, FloatT};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: FloatT
}

impl Camera {
    pub fn new(lookfrom: Point3, 
               lookat: Point3, 
               vup: Vec3,
               vfov: FloatT, 
               aspect_ratio: FloatT,
               aperature: FloatT,
               focus_dist: FloatT) -> Camera {

        // Vertical FOV in degrees
        let theta = (std::f64::consts::PI as FloatT) / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);
        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;
    
        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu: cu,
            cv: cv,
            lens_radius: aperature / 2.0
        }
    }

    pub fn get_ray(&self, s: FloatT, t: FloatT) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }

}