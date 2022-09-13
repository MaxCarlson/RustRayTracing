use super::vec::{Point3, Vec3, FloatT};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: FloatT
}

impl Sphere {
    pub fn new(cen: Point3, r: FloatT) -> Sphere {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find nearest root
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        
        Some(rec)
    }
}
