use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin, direction, time
        }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}