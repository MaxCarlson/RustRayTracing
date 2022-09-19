use super::hit::HitRecord;
use super::vec::{Point3, FloatT};
use super::ray::Ray;
use super::hit::{Hit};

use std::mem;

pub struct AABB {
    min: Point3, 
    max: Point3
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        AABB {
            min: minimum,
            max: maximum
        }
    }

    pub fn min(&self) -> Point3 { self.min }
    pub fn max(&self) -> Point3 { self.max }
}

impl Hit for AABB {

    // Unoptimized hit for AABB
    //fn hit(&self, r: &Ray, mut t_min: FloatT, mut t_max: FloatT) -> bool {
    //    for a in 0..3 {
    //        let t0: FloatT = FloatT::min((self.minimum[a] - r.origin()[a]) / r.direction()[a],
    //                                     (self.maximum[a] - r.origin()[a]) / r.direction()[a]);
    //        let t1: FloatT = FloatT::max((self.minimum[a] - r.origin()[a]) / r.direction()[a],
    //                                     (self.maximum[a] - r.origin()[a]) / r.direction()[a]);
    //        t_min = FloatT::max(t0, t_min);
    //        t_max = FloatT::max(t1, t_max);
    //        if t_max <= t_min { return false; }
    //    }
    //    true
    //}

    fn hit(&self, r: &Ray, mut t_min: FloatT, mut t_max: FloatT) -> (bool, Option<HitRecord>) { 
        for a in 0..3 {
            let invD = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * invD;
            let mut t1 = (self.max()[a] - r.origin()[a]) * invD;
            
            if invD < 0.0 { mem::swap(&mut t0, &mut t1); }
            
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            
            if t_max <= t_min {
                return (false, None);
            }
        }
        (true, None)
    }
}