use std::simd::{f32x2, Simd};
use std::mem;

use crate::{vec3::Point3, ray::Ray};

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(minimum: &Point3, maximum: &Point3) -> AABB {
        AABB { minimum: *minimum, maximum: *maximum }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {

            let b: f32x2 = Simd::from_array([self.minimum[a], self.maximum[a]]);
            let c: f32x2 = f32x2::splat(r.origin[a]);
            let x = b - c;
            let d = f32x2::splat(r.direction[a]);
            let y = x / d;

            // let t0 = f32::min((self.minimum[a] - r.origin[a]) / r.direction[a], (self.maximum[a] - r.origin[a]) / r.direction[a]);
            // let t1 = f32::max((self.minimum[a] - r.origin[a]) / r.direction[a], (self.maximum[a] - r.origin[a]) / r.direction[a]);

            let t0 = f32::min(y[0], y[1]);
            let t1 = f32::max(y[0], y[1]);

            let _t_min = f32::max(t0, t_min);
            let _t_max = f32::min(t1, t_max);

            if _t_max <= _t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f32::min(box0.minimum.x, box1.minimum.x),
            f32::min(box0.minimum.y, box1.minimum.y),
            f32::min(box0.minimum.z, box1.minimum.z)
        );
        let big = Point3::new(
            f32::max(box0.maximum.x, box1.maximum.x),
            f32::max(box0.maximum.y, box1.maximum.y),
            f32::max(box0.maximum.z, box1.maximum.z)
        );
        
        AABB::new(&small, &big)
    }
}