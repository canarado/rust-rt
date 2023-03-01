use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign ,Add, Sub, Mul, Div, RangeInclusive};
use crate::color::Color;
use rand::rngs::ThreadRng;
use rand::Rng;

use std::iter::Sum;

#[derive(Debug, Copy, Clone)]
pub struct Vec3ns {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3ns;

impl Vec3ns {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3ns {
        Vec3ns {
            x, y, z
        }
    }

    pub fn from_slice(slice: [f64; 3]) -> Vec3ns {
        Vec3ns {
            x: slice[0],
            y: slice[1],
            z: slice[2]
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot_product(&self, other: &Vec3ns) -> f64 {
        dot_product(self, other)
    }

    pub fn cross_product(&self, other: &Vec3ns) -> Vec3ns {
        cross_product(self, other)
    }

    pub fn normalized(&self) -> Self {
        unit_vector(self)
    }

    pub fn as_color(&self) -> Color {
        (self[0], self[1], self[2])
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3ns {
        Vec3ns::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>()
        )
    }

    pub fn random_in_range(rng: &mut ThreadRng, range: RangeInclusive<f64>) -> Vec3ns {
        Vec3ns::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone())
        )
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;

        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(&self, other: &Vec3ns) -> Vec3ns {
        reflect(self, other)
    }
}

pub fn unit_vector(vec: &Vec3ns) -> Vec3ns {
    vec / vec.length()
}

pub fn dot_product(v1: &Vec3ns, v2: &Vec3ns) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn cross_product(v1: &Vec3ns, v2: &Vec3ns) -> Vec3ns {
    Vec3ns {
        x: v1[1] * v2[2] - v1[2] * v2[1],
        y: v1[2] * v2[0] - v1[0] * v2[2],
        z: v1[0] * v2[1] - v1[1] * v2[0]
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3ns {
    loop {
        let p: Vec3ns = Vec3ns::random_in_range(rng, -1.0..=1.0);

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3ns {
    unit_vector(&random_in_unit_sphere(rng))
}

pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vec3ns) -> Vec3ns {
    let in_unit_sphere = random_in_unit_sphere(rng);

    if dot_product(&in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere
    } else {
        return -in_unit_sphere
    }
}

pub fn reflect(vec1: &Vec3ns, vec2: &Vec3ns) -> Vec3ns {
    vec1 - 2.0 * dot_product(&vec1, &vec2) * vec2
}

pub fn refract(uv: &Vec3ns, n: &Vec3ns, e: f64) -> Vec3ns {
    let cos_theta = f64::min(dot_product(&-uv, n), 1.0);
    let perp = e * ( uv + cos_theta * n);
    let parr = (1.0 - perp.length_squared()).abs().sqrt().neg() * n;

    perp + parr
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3ns {
    loop {
        let p = Vec3ns::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p
    }
}

impl Neg for Vec3ns {
    type Output = Vec3ns;
    fn neg(self) -> Self::Output {
        Vec3ns {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Neg for &Vec3ns {
    type Output = Vec3ns;

    fn neg(self) -> Self::Output {
        Vec3ns {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Index<u8> for Vec3ns {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3ns index access out of range")
        }
    }
}

impl IndexMut<u8> for Vec3ns {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3ns index access out of range")
        }
    }
}

impl AddAssign for Vec3ns {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn add(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<&Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn add(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<&Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn add(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn add(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn sub(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<&Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn sub(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<&Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn sub(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn sub(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl MulAssign<f64> for Vec3ns {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl MulAssign<isize> for Vec3ns {
    fn mul_assign(&mut self, rhs: isize) {
        let rhs_f = rhs as f64;
        *self *= rhs_f
    }
}

impl Mul<Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn mul(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<&Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn mul(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<&Vec3ns> for Vec3ns {
    type Output = Vec3ns;

    fn mul(self, other: &Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<Vec3ns> for &Vec3ns {
    type Output = Vec3ns;

    fn mul(self, other: Vec3ns) -> Self::Output {
        Vec3ns {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f64> for Vec3ns {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<f64> for &Vec3ns {
    type Output = Vec3ns;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3ns {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3ns> for f64 {
    type Output = Vec3ns;

    fn mul(self, vec: Vec3ns) -> Self::Output {
        vec * self
    }
}

impl Mul<&Vec3ns> for f64 {
    type Output = Vec3ns;

    fn mul(self, vec: &Vec3ns) -> Self::Output {
        vec * self
    }
}

impl DivAssign<f64> for Vec3ns {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
   }
}

impl DivAssign<isize> for Vec3ns {
    fn div_assign(&mut self, rhs: isize) {
        let rhs_f = rhs as f64;
        *self /= rhs_f
    }
}

impl Div<f64> for Vec3ns {
    type Output = Vec3ns;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Div<f64> for &Vec3ns {
    type Output = Vec3ns;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Sum for Vec3ns {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3ns::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}