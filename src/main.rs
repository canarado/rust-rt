use std::rc::Rc;

use raytracer::{
    vec3::*,
    color::*,
    ray::*,
    hittable::*,
    sphere::Sphere, camera::{Camera}, material::{Lambertian, Metal}
};

use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {

    // Program config
    const MAX_RECURSION_DEPTH: u64 = 50;

    // image configuration
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 600;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // world settings aka scene
    let mut world = World::new();

    let ground_mat = Rc::new(Lambertian::new(Vec3::new(0.1, 0.9, 0.0)));
    let ball_mat = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.06));
    let ball2_mat = Rc::new(Lambertian::new(Vec3::new(0.7, 0.0, 0.1)));

    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, ball_mat)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground_mat)));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.5, 1.0), 0.4, ball2_mat)));

    // camera configuration
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(viewport_height, viewport_width, focal_length, origin);

    // RNG cache
    let mut rng = rand::thread_rng();

    // writes header for ppm file to stdout, use with > to pipe to ppm file
    write_ppm_header_to_stdout(IMAGE_WIDTH, IMAGE_HEIGHT);

    // let mut counter: usize = 0;
    // let total = IMAGE_HEIGHT * IMAGE_HEIGHT * SAMPLES_PER_PIXEL;

    for j in (0..(IMAGE_HEIGHT)).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_RECURSION_DEPTH, &mut rng);
            }

            write_color_to_stdout(pixel_color.as_color(), SAMPLES_PER_PIXEL);
        }
    }
}

fn ray_color(ray: Ray, world: &World, depth: u64, rng: &mut ThreadRng) -> Vec3 {

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0)
    }

    // 0.1e-325
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(&ray, &rec) {
            return attenuation * ray_color(scattered, world, depth - 1, rng)
        } else {
            return Vec3::new(0.0, 0.0, 0.0)
        }
        // let target = rec.p + random_in_hemisphere(rng, rec.normal);
        // return 0.5 * ray_color(Ray::new(rec.p, target), world, depth - 1, rng);
    }

    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}