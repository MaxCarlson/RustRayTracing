use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

type FloatT = f64;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [FloatT; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: FloatT, e1: FloatT, e2: FloatT) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = FloatT;

    fn index(&self, index: usize) -> &FloatT {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut FloatT {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        };
    }
}

impl Mul<FloatT> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: FloatT) -> Vec3 {
        Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other]
        }
    }
}

impl MulAssign<FloatT> for Vec3 {
    fn mul_assign(&mut self, other: FloatT) -> () {
        *self = Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other]
        };
    }
}

impl Mul<Vec3> for FloatT {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other[0], self * other[1], self * other[2]]
        }
    }
}

impl Div<FloatT> for Vec3 {
    type Output = Vec3;

    fn div(self, other: FloatT) -> Vec3 {
        Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other]
        }
    }
}

impl DivAssign<FloatT> for Vec3 {
    fn div_assign(&mut self, other: FloatT) -> () {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other]
        };
    }
}
