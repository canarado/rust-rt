use std::sync::Arc;

use crate::{material::Scatter, vec3::{Vec3, cross_product}, hittable::{Hit, HitRecord}, aabb::AABB, ray::Ray};

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatter>
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<dyn Scatter>) -> Triangle {
        Triangle {
            v0, v1, v2, normal: cross_product(&(v1 - v0), &(v2 - v0)), material
        }
    }

    pub fn new_with_normal(v0: Vec3, v1: Vec3, v2: Vec3, normal: Vec3, material: Arc<dyn Scatter>) -> Triangle {
        Triangle {
            v0, v1, v2, normal, material
        }
    }
}

impl Hit for Triangle {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            &Vec3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)),
                self.v0.y.min(self.v1.y.min(self.v2.y)),
                self.v0.z.min(self.v1.z.min(self.v2.z))
            ),
            &Vec3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)),
                self.v0.y.max(self.v1.y.max(self.v2.y)),
                self.v0.z.max(self.v1.z.max(self.v2.z))
            )
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;

        let pvec = r.direction.cross_product(&v0v2);
        let det = v0v1.dot_product(&pvec);

        if det.abs() < 1e-4 {
            return None;
        }

        let inv_det = 1. / det;

        let tvec = r.origin - self.v0;
        let u = tvec.dot_product(&pvec) * inv_det;
        if u < 0. || u > 1. {
            return None
        }

        let qvec = tvec.cross_product(&v0v1);
        let v = r.direction.dot_product(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None
        }

        let t = v0v2.dot_product(&qvec) * inv_det;

        if t < t_min || t > t_max {
            return None
        }

        let p = r.at(t);

        return Some(HitRecord {
            u,
            v,
            t,
            p,
            normal: self.normal,
            mat: self.material.clone(),
            front_face: false
        })
    }
}