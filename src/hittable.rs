use std::{sync::Arc};

use crate::{vec3::*, ray::Ray, material::Scatter, aabb::AABB};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot_product(&r.direction, &outward_normal) < 0.0;
        
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = -outward_normal
        }
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        match self.first() {
            Some(first) =>
                match first.bounding_box(time0, time1) {
                    Some(bbox) =>
                        self.iter().skip(1).try_fold(bbox, |acc, hitable|
                            match hitable.bounding_box(time0, time1) {
                                Some(bbox) => Some(AABB::surrounding_box(&acc, &bbox)),
                                _ => None
                            }
                        ),
                    _ => None
                },
            _ => None
        }
    }
}