use std::{sync::Arc, simd::{Simd, f32x2}, f32::consts::PI};

use crate::{vec3::{Point3, dot_product, Vec3}, hittable::{Hit, HitRecord}, ray::Ray, material::Scatter, aabb::AABB};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub mat: Arc<dyn Scatter>
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center, radius, mat: material
        }
    }

    pub fn get_uv(p: &Vec3) -> (f32, f32) {
        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + PI;

       let u = phi / (2. * PI);
       let v = theta / PI;

       (u, v)
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
            u: 0.,
            v: 0.,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;

        let (u, v) = Sphere::get_uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(
            AABB::new(
                &(self.center - Vec3::new(self.radius, self.radius, self.radius)),
                &(self.center + Vec3::new(self.radius, self.radius, self.radius))
            )
        )
    }
}

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat: Arc<dyn Scatter>
}

impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, radius: f32, material: Arc<dyn Scatter>, time0: f32, time1: f32) -> MovingSphere {
        MovingSphere {
            center0, center1, radius, mat: material, time0, time1
        }
    }

    pub fn center(&self, time: &f32) -> Point3 {

        // not sure if the SIMD instructions actually help here, profiling was hit or miss on CPU timing
        let sub1: f32x2 = Simd::from_array([*time, self.time1]);
        let sub2: f32x2 = Simd::from_array([self.time0, self.time0]);
        let r = sub1 - sub2;

        &self.center0 + r[0] / r[1] * (&self.center1 - &self.center0)
        // &self.center0 + ((time - &self.time0) / (&self.time1 - &self.time0)) * (&self.center1 - &self.center0)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
            u: 0.,
            v: 0.,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false
        };

        let outward_normal = (rec.p - self.center(&r.time)) / self.radius;

        let (u, v) = Sphere::get_uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            &(self.center(&time0) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(&time0) + Vec3::new(self.radius, self.radius, self.radius))
        );

        let box1 = AABB::new(
            &(self.center(&time1) - Vec3::new(self.radius, self.radius, self.radius)),
            &(self.center(&time1) + Vec3::new(self.radius, self.radius, self.radius))
        );

        Some(AABB::surrounding_box(&box0, &box1))
    }
}