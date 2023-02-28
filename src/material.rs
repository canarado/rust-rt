use crate::{ray::Ray, hittable::HitRecord, vec3::{random_in_unit_sphere, Vec3, unit_vector, refract, dot_product, reflect}};
use rand::Rng;

pub trait Scatter: Sync+Send {
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
        let reflected = r.direction.reflect(&rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(&mut rand::thread_rng()));

        if scattered.direction.dot_product(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    pub fn reflectance(cosine: f64, r: f64) -> f64 {
        let mut r0 = (1.0 - r) / (1.0 + r);
        r0 *= r0;

        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refract_ratio: f64;

        if rec.front_face {
            refract_ratio = 1.0 / self.ir;
        } else {
            refract_ratio = self.ir;
        }

        let unit_dir = unit_vector(&r.direction);
        let cos_theta = f64::min(dot_product(&-unit_dir, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refract_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, refract_ratio) > rand::thread_rng().gen::<f64>() {
            direction = reflect(&unit_dir, &rec.normal);
        } else {
            direction = refract(&unit_dir, &rec.normal, refract_ratio);
        }

        Some(
            (attenuation, Ray::new(rec.p, direction))
        )
    }
}