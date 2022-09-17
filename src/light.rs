use super::vec::{Point3, Vec3, FloatT, Color};
use super::ray::Ray;
use super::hit::{HitRecord, World};

pub struct Light {
    p: Point3,
    c: Color,
    dir: Vec3,
    l: FloatT
}

impl Light {
    pub fn new(point: Point3, color: Color, direction: Vec3, lumonocity: FloatT) -> Light {
        Light {
            p: point,
            c: color,
            dir: direction,
            l: lumonocity
        }
    }
}

pub trait LightHit {
    fn get_color(&self, rec: &HitRecord, world: &World) -> Color;
}

pub type Lights = Vec<Box<Light>>;

impl LightHit for Lights {
    fn get_color(&self, rec: &HitRecord, world: &World) -> Color {
        let mut origin: Point3 = rec.p;
        let mut color: Color = Color::default();

        for l in self {
            let dir: Vec3 = l.p - rec.p;
            let r = Ray::new(origin, dir);
            let len = dir.length();

            for o in world {
                if let Some(trec) = o.hit(&r, 0.0001, len) {
                    return Color::default();
                }
            }
            color += l.c * l.l;
        }
        color
    }
}

//pub trait Hit : Send + Sync {
//    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> Option<HitRecord>;
//}
//
//pub type World = Vec<Box<dyn Hit>>;
//
//impl Hit for World {
//    fn hit(&self, r: &Ray, t_min: FloatT, t_max: FloatT) -> Option<HitRecord> {
//        let mut tmp_rec = None;
//        let mut closest = t_max;
//
//        // Find the closest object that the ray intersects
//        for object in self {
//            if let Some(rec) = object.hit(r, t_min, closest) {
//                closest = rec.t;
//                tmp_rec = Some(rec);
//            }
//        }
//        tmp_rec
//    }
//}