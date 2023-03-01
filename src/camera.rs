use crate::{vec3::{Point3, Vec3, unit_vector, cross_product, random_in_unit_disk}, ray::Ray};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

unsafe impl Sync for Camera {}
unsafe impl Send for Camera {}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64, origin: Point3) -> Camera {
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin,
            horizontal: h,
            vertical: v,
            lower_left_corner: origin - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, focal_length)
        }
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

#[derive(Copy, Clone)]
pub struct OrthographicCamera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub uvw: [Vec3; 3],
    pub lens_radius: f64
}

unsafe impl Sync for OrthographicCamera {}
unsafe impl Send for OrthographicCamera {}

impl OrthographicCamera {
    pub fn new(origin: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> OrthographicCamera {
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
            origin: origin, lower_left_corner, horizontal, vertical, uvw: [u, v, w], lens_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(&mut rand::thread_rng());
        let offset = self.uvw[0] * rd.x + self.uvw[1] * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}