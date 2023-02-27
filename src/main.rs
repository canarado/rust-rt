use std::{rc::Rc, time::Instant};

use raytracer::{
    vec3::*,
    color::*,
    ray::*,
    hittable::*,
    sphere::Sphere, camera::{OrthographicCamera}, material::{Lambertian, Metal, Dielectric}
};

use rand::Rng;
use rand::rngs::ThreadRng;

use indicatif::ProgressBar;

use rayon::prelude::*;

fn main() {
    
    let start = Instant::now();
    // Program config
    const MAX_RECURSION_DEPTH: u64 = 50;
    
    // image configuration
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 600;
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u64 = 600;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 25;
    
    // RNG cache
    let mut rng = rand::thread_rng();

    let world = demo(&mut rng);
    // let world = simple_test();

    // let origin = Point3::new(1.0, 3.3, -3.0);
    // let lookat = Point3::new(0.0, 0.0, 0.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = 1.0;
    // let aperture = 0.3;

    // let camera = OrthographicCamera::new(origin, lookat, vup, 45.0, ASPECT_RATIO, aperture, dist_to_focus);

    let origin = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = OrthographicCamera::new(origin, lookat, vup, 45.0, ASPECT_RATIO, aperture, dist_to_focus);

    // writes header for ppm file to stdout, use with > to pipe to ppm file
    write_ppm_header_to_stdout(IMAGE_WIDTH, IMAGE_HEIGHT);

    let bar = ProgressBar::new(IMAGE_HEIGHT * IMAGE_WIDTH);

    let mut count: u64 = 0;
    let mut pixel_list: Vec<String> = Vec::new();

    for j in (0..(IMAGE_HEIGHT)).rev() {
        for i in 0..IMAGE_WIDTH {
            count += 1;
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_RECURSION_DEPTH, &mut rng);
            }

            //write_color_to_stdout(pixel_color.as_color(), SAMPLES_PER_PIXEL);
            write_color_to_list(&mut pixel_list, pixel_color.as_color(), SAMPLES_PER_PIXEL);
            if count % 500 == 0 {
                bar.inc(500);
            }
        }
    }

    write_vector_to_stdout(&mut pixel_list);

    bar.finish();

    eprintln!("Render Time: {:.2?}", start.elapsed());
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
    }

    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn demo(rng: &mut ThreadRng) -> World {
    let mut world = World::new();

    let ground_mat = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    // y may need inverted, we shall see
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat));
    world.push(ground);

    for a in 0..=22 {
        for b in 0..=22 {
            let af = a as f64 - 11.0;
            let bf = b as f64 - 11.0;

            let c = rng.gen::<f64>();
            let center = Point3::new(af + 0.9 * rng.gen::<f64>(), 0.2, bf + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if c < 0.8 {
                    let albedo = Vec3::random(rng) * Vec3::random(rng);
                    let mat = Rc::new(Lambertian::new(albedo));
                    let obj = Box::new(Sphere::new(center, 0.2, mat));

                    world.push(obj);
                }
            } else if c < 0.95 {
                let albedo = Vec3::random_in_range(rng, 0.5..=1.0);
                let fuzz = rng.gen_range(0.0..=0.5);
                let mat = Rc::new(Metal::new(albedo, fuzz));
                let obj = Box::new(Sphere::new(center, 0.2, mat));

                world.push(obj);
            } else {
                let mat = Rc::new(Dielectric::new(1.5));
                let obj = Box::new(Sphere::new(center, 0.2, mat));

                world.push(obj);
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.push(
        Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1))
    );

    let mat2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(
        Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2))
    );

    let mat3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(
        Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3))
    );

    world
}

pub fn simple_test() -> World {
    let mut world = World::new();

    let mat = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let sphere = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, mat));

    world.push(sphere);

    world
}