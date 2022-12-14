use super::vec::{Point3, Vec3, FloatT};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::material::Scatter;

use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: FloatT,
    mat: Arc<dyn Scatter>
}

impl Sphere {
    pub fn new(cen: Point3, r: FloatT, m: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: m
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> (bool, Option<HitRecord>) {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return (false, None);
        }

        // Find nearest root
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return (false, None);
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        
        (true, Some(rec))
    }
}
