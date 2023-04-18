use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{vec3::{Point3, Vec3, unit_vector, cross_product, random_in_unit_disk}, ray::Ray, config::Config};

#[derive(Serialize, Deserialize)]
pub enum CameraType {
    Perspective,
    Orthographic,
}

pub trait Camera: Sync + Send {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut rng = rand::thread_rng();
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin, rng.gen_range(self.time0..=self.time1))
    }
}

impl Camera for OrthographicCamera {
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * random_in_unit_disk(&mut rng);
        let offset = self.uvw[0] * rd.x + self.uvw[1] * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset, rng.gen_range(self.time0..=self.time1))
    }
}

pub fn create_camera(config: &Config, image_width: u32, image_height: u32) -> Box<dyn Camera> {
    let aspect_ratio = image_width as f64 / image_height as f64;

    match config.camera.typ {
        CameraType::Perspective => {
            let camera = PerspectiveCamera::new(
                image_height.into(),
                image_width.into(),
                config.camera.focal_length,
                config.camera.origin,
                config.camera.time0,
                config.camera.time1,
            );
            Box::new(camera)
        }
        CameraType::Orthographic => {
            let camera = OrthographicCamera::new(
                config.camera.origin,
                config.camera.lookat,
                config.camera.vup,
                config.camera.vfov,
                aspect_ratio,
                config.camera.aperture,
                config.camera.dist_to_focus,
                config.camera.time0,
                config.camera.time1,
            );
            Box::new(camera)
        }
    }
}

#[derive(Copy, Clone)]
pub struct PerspectiveCamera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub time0: f64,
    pub time1: f64
}

impl PerspectiveCamera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64, origin: Point3, time0: f64, time1: f64) -> PerspectiveCamera {
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);
        PerspectiveCamera {
            origin,
            horizontal: h,
            vertical: v,
            lower_left_corner: origin - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, focal_length),
            time0,
            time1
        }
    }
}

#[derive(Copy, Clone)]
pub struct OrthographicCamera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub uvw: [Vec3; 3],
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64
}

impl OrthographicCamera {
    pub fn new(origin: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> OrthographicCamera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let v_height = 2.0 * h;
        let v_width = aspect_ratio * v_height;

        let w = unit_vector(&(origin - lookat));
        let u = unit_vector(&cross_product(&vup, &w));
        let v = cross_product(&w, &u);

        let horizontal = focus_dist * v_width * u;
        let vertical = focus_dist * v_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        OrthographicCamera {
            origin, lower_left_corner, horizontal, vertical, uvw: [u, v, w], lens_radius, time1, time0
        }
    }
}
