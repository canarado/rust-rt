#![feature(let_chains)]
#![allow(unused_doc_comments, non_snake_case)]

use std::{time::Instant, sync::Arc, fs::File, io::BufWriter, path::Path};

use raytracer::{
    vec3::*,
    color::*,
    ray::*,
    hittable::*,
    sphere::{Sphere, MovingSphere}, camera::{OrthographicCamera}, material::{Lambertian, Metal, Dielectric}
};

use rand::Rng;
use rand::rngs::ThreadRng;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use png;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Simple Toy Raytracer written in Rust", long_about = None)]
struct Args {
    /// number of threads
    #[arg(short, default_value_t = 1)]
    threads: u8,

    /// number of samples per pixel
    #[arg(short, default_value_t = 100)]
    samples: u16,

    #[arg(short, default_value_t = 1280, value_name = "width")]
    image_width: u32,

    #[arg(short, default_value_t = 720, value_name = "height")]
    image_height: u32,


}

fn main() {

    let args = Args::parse();

    rayon::ThreadPoolBuilder::new().num_threads(args.threads.into()).build_global().unwrap();
    
    let start = Instant::now();
    // Program config
    const MAX_RECURSION_DEPTH: u64 = 50;
    
    // image configuration
    let IMAGE_WIDTH: u32 = args.image_width.into();
    let IMAGE_HEIGHT: u32 = args.image_height.into();
    let SAMPLES_PER_PIXEL: u64 = args.samples.into();
    let ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

    let world = demo();

    let origin = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = OrthographicCamera::new(origin, lookat, vup, 45.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

    // writes header for ppm file to stdout, use with > to pipe to ppm file
    // write_ppm_header_to_stdout(IMAGE_WIDTH, IMAGE_HEIGHT);

    // PNG setup
    let output_file_path = Path::new(r"C:\Users\canny-dev\Documents\rust-rt\test.png");
    let file = File::create(output_file_path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(IMAGE_HEIGHT / IMAGE_WIDTH * 100000));
    encoder.set_source_chromaticities(png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    ));
    let mut writer = encoder.write_header().unwrap();

    /**
     * image should be a vector of <color1.0, color1.1, color1.2, color.0, ...>
     * 
     * iter over height, map values
     *   iter over width, map values
     *     pixel color
     *   take pixel colors and collect them into a vector of individual color components
     * take vector of color components, and collect it all to a 1d vector of color components
     * 
     * take the 1d vector and chunk(3), write formatted {} {} {}, Color; to a list, print list to stdout
     */

    let mut list =
        (0..IMAGE_HEIGHT).rev().collect::<Vec<u32>>().into_par_iter().progress_count(IMAGE_HEIGHT as u64).flat_map(|j| {
            (0..IMAGE_WIDTH).flat_map(|i| {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(r, &world, MAX_RECURSION_DEPTH, &mut rng);
                }

                [pixel_color.x, pixel_color.y, pixel_color.z]
            }).collect::<Vec<f64>>()
        }).collect::<Vec<f64>>();

    let sampled = apply_samples(&mut list, SAMPLES_PER_PIXEL, IMAGE_HEIGHT, IMAGE_WIDTH);

    writer.write_image_data(&sampled).unwrap();

    eprintln!("Render Time: {:.2?}", start.elapsed());
}

fn ray_color(ray: Ray, world: &World, depth: u64, rng: &mut ThreadRng) -> Vec3 {

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0)
    }

    // 0.1e-325
    if let Some(rec) = world.hit(&ray, 0.001, f64::INFINITY) {
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

pub fn demo() -> World {
    let rng = &mut rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat));
    world.push(ground);

    for a in 0..=22 {
        for b in 0..=22 {
            let af = a as f64 - 11.0;
            let bf = b as f64 - 11.0;

            let c = rng.gen::<f64>();
            let center = Point3::new(af + 0.9 * rng.gen::<f64>(), 0.2, bf + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if c < 0.45 {
                    let albedo = Vec3::random(rng) * Vec3::random(rng);
                    let mat = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..=0.5), 0.0);

                    let obj = Box::new(MovingSphere::new(center, center2, 0.2, mat, 0.0, 1.0));

                    world.push(obj);
                } else if c < 0.75 {
                    let albedo = Vec3::random_in_range(rng, 0.5..=1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    let obj = Box::new(Sphere::new(center, 0.2, mat));

                    world.push(obj);
                } else {
                    let mat = Arc::new(Dielectric::new(1.5));
                    let obj = Box::new(Sphere::new(center, 0.2, mat));

                    world.push(obj);
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.push(
        Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1))
    );

    let mat2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(
        Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2))
    );

    let mat3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(
        Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3))
    );

    world
}

pub fn simple_test() -> World {
    let mut world = World::new();

    let mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let sphere = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, mat));

    world.push(sphere);

    world
}