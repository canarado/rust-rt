use raytracer::{
    vec3::*,
    color::*,
    ray::*,
    hittable::*,
    sphere::Sphere, camera::{Camera}
};

use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {

    // Program config
    const MAX_RECURSION_DEPTH: u64 = 50;

    // image configuration
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // world settings aka scene
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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

    for j in (0..(IMAGE_HEIGHT)).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let r = camera.get_ray(u, v);
                let c = ray_color(r, &world, MAX_RECURSION_DEPTH, &mut rng);
                pixel_color += c;
            }

            write_color_to_stdout(pixel_color.as_color(), SAMPLES_PER_PIXEL);
        }
    }
}

fn ray_color(ray: Ray, world: &World, depth: u64, rng: &mut ThreadRng) -> Vec3 {

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0)
    }

    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        return 0.5 * ray_color(Ray::new(rec.p, target), world, depth - 1, rng);
        // return (Vec3::new(1.0, 1.0, 1.0) + rec.normal) * 0.5
    }

    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}