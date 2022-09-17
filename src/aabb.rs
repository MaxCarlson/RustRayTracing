use super::vec::{Point3, Vec3};


pub struct AABB {
    minimum: Point3, 
    maximum: Point3
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        AABB {
            minimum: minimum,
            maximum: maximum
        }
    }

    pub fn min(self) -> Point3 { self.minimum }
    pub fn max(self) -> Point3 { self.maximum }
}