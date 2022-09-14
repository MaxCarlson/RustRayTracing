use super::vec::{Vec3, Point3, FloatT};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: FloatT = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: FloatT = 2.0;
        const VIEWPORT_WIDTH: FloatT = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: FloatT = 1.0;

        let orig = Point3::default();
        let h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            origin: orig,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc
        }
    }

    pub fn get_ray(&self, u: FloatT, v: FloatT) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

}