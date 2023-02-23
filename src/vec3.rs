use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign ,Add, Sub, Mul, Div, RangeInclusive};
use crate::color::Color;
use rand::rngs::ThreadRng;
use rand::Rng;

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

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot_product(self, other: Vec3) -> f64 {
        dot_product(self, other)
    }

    pub fn cross_product(self, other: Vec3) -> Vec3 {
        cross_product(self, other)
    }

    pub fn normalized(self) -> Self {
        unit_vector(self)
    }

    pub fn as_color(self) -> Color {
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

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;

        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, other: Vec3) -> Vec3 {
        reflect(self, other)
    }
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

pub fn dot_product(v1: Vec3, v2: Vec3) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

pub fn cross_product(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1[1] * v2[2] - v1[2] * v2[1],
        y: v1[2] * v2[0] - v1[0] * v2[2],
        z: v1[0] * v2[1] - v1[1] * v2[0]
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
    unit_vector(random_in_unit_sphere(rng))
}

pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);

    if dot_product(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere
    } else {
        return -in_unit_sphere
    }
}

pub fn reflect(vec1: Vec3, vec2: Vec3) -> Vec3 {
    vec1 - 2.0 * dot_product(vec1, vec2) * vec2
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
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
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
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<u64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        vec * self
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
        (1.0 / rhs) * self
    }
}

pub fn clock_angle(time: &'static str) -> i32 {
    // split the input into an array
    let s = time.split(":").collect::<Vec<&str>>();

    // parse hour to an int
    let mut hour = s[0].parse::<i32>().unwrap();

    // check if hour is greater than 12 and if so, subtract 12 to convert to a 12 hour radial clock
    if hour > 12 {
        hour -= 12
    }
    // convert hour to a clockwise angle relative to 00:00
    hour = hour * 30;

    // parse minute to an int and convert to a angle relative to 00:00 
    let minute = s[1].parse::<i32>().unwrap() * 6;

    // account for extra rotation from minute offset the hour hand
    hour += ((minute / 6) as f32 * 0.5) as i32;

    // get the difference between the two angles by which is largest, and return the absolute value
    (std::cmp::max(hour, minute) - std::cmp::min(hour, minute)).abs()
}