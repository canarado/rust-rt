use crate::{vec3::{Point3, Vec3}, ray::Ray};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

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