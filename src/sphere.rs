use std::{sync::Arc, simd::{Simd, f64x2}};

use crate::{vec3::{Point3, dot_product, Vec3}, hittable::{Hit, HitRecord}, ray::Ray, material::Scatter};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center, radius, mat: material
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - self.center;
        let a = &r.direction.length_squared();
        let half_b = dot_product(&oc, &r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let fd = &half_b*&half_b;
        let sd = a * &c;
        let discriminant = &fd - &sd;

        //let discriminant = half_b*half_b - a*c;
        if &discriminant < &0.0 {
            return None;
        }

        let sqrtd = &discriminant.sqrt();
        let mut root = (-&half_b - sqrtd) / a;

        if &root < &t_min || &t_max < &root {
            root = (-&half_b + sqrtd) / a;
            if &root < &t_min || &t_max < &root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }
}

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>
}

impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, radius: f64, material: Arc<dyn Scatter>, time0: f64, time1: f64) -> MovingSphere {
        MovingSphere {
            center0, center1, radius, mat: material, time0, time1
        }
    }

    pub fn center(&self, time: &f64) -> Point3 {

        // not sure if the SIMD instructions actually help here, profiling was hit or miss on CPU timing
        let sub1: f64x2 = Simd::from_array([*time, self.time1]);
        let sub2: f64x2 = Simd::from_array([self.time0, self.time0]);
        let r = sub1 - sub2;

        &self.center0 + r[0] / r[1] * (&self.center1 - &self.center0)
        // &self.center0 + ((time - &self.time0) / (&self.time1 - &self.time0)) * (&self.center1 - &self.center0)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - &self.center(&r.time);
        let a = r.direction.length_squared();
        let half_b = dot_product(&oc, &r.direction);
        let c = &oc.length_squared() - &self.radius * &self.radius;

        let discriminant = &half_b * &half_b - &a * &c;

        if &discriminant < &0.0 {
            return None;
        }

        let sqrtd = &discriminant.sqrt();
        let mut root = (-&half_b - sqrtd) / &a;

        if &root < &t_min || &t_max < &root {
            root = (-&half_b + sqrtd) / &a;
            if &root < &t_min || &t_max < &root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center(&r.time)) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}