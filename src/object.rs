use super::aabb::AABB;

pub trait Object : Send + Sync {
    fn hit(&self) -> AABB;
}