use crate::{vec3::{Vec3, Point3}, camera::CameraType};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {
    pub typ: CameraType,
    pub origin: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub dist_to_focus: f64,
    pub aperture: f64,
    pub vfov: f64,
    pub focal_length: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            typ: CameraType::Orthographic,
            origin: Point3::new(120.0, 10.0, 120.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            dist_to_focus: 20.0,
            focal_length: 35.0,
            aperture: 0.1,
            vfov: 45.0,
            time0: 0.0,
            time1: 1.0
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub camera: CameraConfig,
}
