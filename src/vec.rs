use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};
use std::fmt;
use std::fmt::Display;
use rand::{Rng, thread_rng};


pub type FloatT = f64;

#[derive(Default, Clone, Copy)]
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

    pub fn x(self) -> FloatT {
        self[0]
    }

    pub fn y(self) -> FloatT {
        self[1]
    }

    pub fn z(self) -> FloatT {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> FloatT {
        return self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(self) -> FloatT {
        return self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x()
            ]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    
    // 𝜃′= sin^-1(𝜂/𝜂′⋅sin𝜃)
    pub fn refract(self, n: Vec3, eta_over_etap: FloatT) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);

        // 𝐑′⊥ = 𝜂/𝜂′(𝐑+(−𝐑⋅𝐧)𝐧)
        let r_out_perp = eta_over_etap * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random(r: Range<FloatT>) -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            e: [rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone())]
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();

        // In the same hemisphere as normal
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn random_in_unit_disc() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(self) -> bool {
        const EPS: FloatT = 1.0e-8;
        self.x().abs() < EPS && self.y().abs() < EPS && self.z().abs() < EPS
    }
    
    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self.x() / (samples_per_pixel as FloatT)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.y() / (samples_per_pixel as FloatT)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.z() / (samples_per_pixel as FloatT)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
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

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl Sub<FloatT> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: FloatT) -> Vec3 {
        Vec3 {
            e: [self[0] - other, self[1] - other, self[2] - other]
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]]
        }
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

impl Mul<Vec3> for u64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self as FloatT * other[0], self as FloatT * other[1], self as FloatT * other[2]]
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
impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
