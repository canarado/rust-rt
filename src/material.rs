use crate::{ray::Ray, hittable::HitRecord, vec3::{random_in_unit_sphere, Vec3}};

pub trait Scatter {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere(&mut rand::thread_rng()).normalized();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo, fuzz
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r.direction.reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(&mut rand::thread_rng()));

        if scattered.direction.dot_product(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}