use std::{path::Path, sync::Arc};

use tobj;

use crate::{hittable::{Hit, World}, vec3::Vec3, material::{Dielectric, Metal, Scatter, Lambertian}, texture::ConstantTexture, triangle::Triangle, bvh::BVH};

pub fn load_obj_and_position(path: &Path) -> BVH {
    let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

    let (models, mats) = obj.unwrap();
    let mut world: World = Vec::new();

    let default_material = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(0.6, 0.6, 0.6))));

    let materials: Vec<Arc<dyn Scatter>> = mats.unwrap().iter().map(|m| {
        let mat: Arc<dyn Scatter> = match m.illumination_model {
            Some(7) => Arc::new(Dielectric::new(m.optical_density as f64)),
            Some(5) => Arc::new(Metal::new(Vec3::new(m.diffuse[0] as f64, m.diffuse[1] as f64, m.diffuse[2] as f64), (1. / m.shininess) as f64)),
            _ => Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(m.diffuse[0] as f64, m.diffuse[1] as f64, m.diffuse[2] as f64))))
        };

        mat
    }).collect();

    for m in models.iter() {
        let mesh = &m.mesh;

        for f in mesh.indices.chunks(3) {
            let v0 = Vec3::new(mesh.positions[f[0] as usize * 3] as f64, mesh.positions[f[0] as usize * 3 + 1] as f64, mesh.positions[f[0] as usize * 3 + 2] as f64);
            let v1 = Vec3::new(mesh.positions[f[1] as usize * 3] as f64, mesh.positions[f[1] as usize * 3 + 1] as f64, mesh.positions[f[1] as usize * 3 + 2] as f64);
            let v2 = Vec3::new(mesh.positions[f[2] as usize * 3] as f64, mesh.positions[f[2] as usize * 3 + 1] as f64, mesh.positions[f[2] as usize * 3 + 2] as f64);

            let mat = match mesh.material_id {
                Some(id) => Arc::clone(&materials[id]),
                None => Arc::clone(&default_material)
            };

            let tri: Triangle;

            if mesh.normals.len() > 0 {
                let normal = Vec3::new(mesh.normals[f[0] as usize * 3] as f64, mesh.normals[f[1] as usize * 3] as f64, mesh.normals[f[2] as usize * 3] as f64);
                tri = Triangle::new_with_normal(v0, v1, v2, normal, mat);
            } else {
                tri = Triangle::new(v0, v1, v2, Arc::clone(&mat));
            }

            world.push(Box::new(tri));
        }
    }

    BVH::new(world, 0.0, 1.0)

}

pub fn add_obj_to_world(world: &mut World, obj: BVH, _position: Vec3) {
    world.push(Box::new(obj));
}