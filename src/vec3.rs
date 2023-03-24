use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign ,Add, Sub, Mul, Div, RangeInclusive};
use crate::color::Color;
use rand::rngs::ThreadRng;
use rand::Rng;

use std::simd::{f64x4, Simd};

use std::iter::Sum;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x, y, z
        }
    }

    pub fn from_slice(slice: [f64; 3]) -> Vec3 {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2]
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let x: f64x4 = Simd::from_array([self[0], self[1], self[2], 0.0]);
        let y = &x * &x;

        &y[0] + &y[1] + &y[2]
    }

    pub fn dot_product(&self, other: &Vec3) -> f64 {
        dot_product(self, other)
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        cross_product(self, other)
    }

    pub fn normalized(&self) -> Self {
        unit_vector(self)
    }

    pub fn as_color(&self) -> Color {
        (self[0], self[1], self[2])
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(
            rng.gen::<f64>(),
            rng.gen::<f64>(),
            rng.gen::<f64>()
        )
    }

    pub fn random_in_range(rng: &mut ThreadRng, range: RangeInclusive<f64>) -> Vec3 {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone())
        )
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;

        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(&self, other: &Vec3) -> Vec3 {
        reflect(self, other)
    }
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.length()
}

pub fn dot_product(v1: &Vec3, v2: &Vec3) -> f64 {
    let x: f64x4 = Simd::from_array([v1[0], v1[1], v1[2], 0.0]);
    let y: f64x4 = Simd::from_array([v2[0], v2[1], v2[2], 0.0]);

    let z = &x * &y;
    &z[0] + &z[1] + &z[2]
}

pub fn cross_product(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let x1: f64x4 = Simd::from_array([v1[1], v1[2], v1[0], 0.0]);
    let y1: f64x4 = Simd::from_array([v2[2], v2[0], v2[1], 0.0]);

    let z1 = x1 * y1;

    let x2: f64x4 = Simd::from_array([v1[2], v1[0], v1[1], 0.0]);
    let y2: f64x4 = Simd::from_array([v2[1], v2[2], v2[0], 0.0]);

    let z2 = x2 * y2;

    let w = z1 - z2;

    Vec3 {
        x: w[0],
        y: w[1],
        z: w[2]
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p: Vec3 = Vec3::random_in_range(rng, -1.0..=1.0);

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    unit_vector(&random_in_unit_sphere(rng))
}

pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);

    if dot_product(&in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere
    } else {
        return -in_unit_sphere
    }
}

pub fn reflect(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
    vec1 - 2.0 * dot_product(&vec1, &vec2) * vec2
}

pub fn refract(uv: &Vec3, n: &Vec3, e: f64) -> Vec3 {
    let cos_theta = f64::min(dot_product(&-uv, n), 1.0);
    let perp = e * ( uv + cos_theta * n);
    let parr = (1.0 - perp.length_squared()).abs().sqrt().neg() * n;

    perp + parr
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Index<u8> for Vec3 {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index access out of range")
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index access out of range")
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x + y;

        *self = Self {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x + y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x + y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x + y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x + y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x - y;

        Vec3 {
            x: z[0],
            y: z[1], 
            z: z[2]
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x - y;

        Vec3 {
            x: z[0],
            y: z[1], 
            z: z[2]
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x - y;

        Vec3 {
            x: z[0],
            y: z[1], 
            z: z[2]
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x - y;

        Vec3 {
            x: z[0],
            y: z[1], 
            z: z[2]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl MulAssign<isize> for Vec3 {
    fn mul_assign(&mut self, rhs: isize) {
        let rhs_f = rhs as f64;
        *self *= rhs_f
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x * y;

        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x * y;

        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x * y;

        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y: f64x4 = Simd::from_array([other.x, other.y, other.z, 0.0]);

        let z = x * y;

        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y = f64x4::splat(rhs);

        let z = x * y;

        Self {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64x4 = Simd::from_array([self.x, self.y, self.z, 0.0]);
        let y = f64x4::splat(rhs);

        let z = x * y;

        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([vec.x, vec.y, vec.z, 0.0]);
        let y: f64x4 = f64x4::splat(self);

        let z = x * y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: &Vec3) -> Self::Output {
        let x: f64x4 = Simd::from_array([vec.x, vec.y, vec.z, 0.0]);
        let y: f64x4 = f64x4::splat(self);

        let z = x * y;
        
        Vec3 {
            x: z[0],
            y: z[1],
            z: z[2]
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
   }
}

impl DivAssign<isize> for Vec3 {
    fn div_assign(&mut self, rhs: isize) {
        let rhs_f = rhs as f64;
        *self /= rhs_f
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}