use super::vec::{Point3, Vec3, FloatT};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::aabb::AABB;

use std::rc::Rc;

pub struct BVHNode {

    left: Rc<BVHNode>,
    right: Rc<BVHNode>,
    bbox: AABB
}

pub struct BVHTree
{
    
}
