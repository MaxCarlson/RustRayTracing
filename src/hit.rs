use super::vec::{Vec3, Point3, FloatT};
use super::ray::Ray;
use super::material::Scatter;
use std::sync::Arc;


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t: FloatT,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        // If the ray dot our normal < 0, the ray is coming from the outside
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
}

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest = t_max;

        // Find the closest object that the ray intersects
        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest) {
                closest = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}
