use std::{path::Path, sync::Arc};

use tobj;

use crate::{hittable::{Hit, World}, vec3::Vec3, material::{Dielectric, Metal, Scatter, Lambertian}, texture::ConstantTexture, triangle::Triangle, bvh::BVH};

pub fn load_obj_and_position(path: &Path) -> BVH {
    let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

    let (models, mats) = obj.unwrap();
    let mut world: World = Vec::new();

    let default_material = Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(0.6, 0.6, 0.6))));

    let materials: Vec<Arc<dyn Scatter>> = match mats {
        Ok(mmats) => {
            mmats.iter().map(|m| {
                let mat: Arc<dyn Scatter> = match m.illumination_model {
                    Some(7) => Arc::new(Dielectric::new(m.optical_density as f32)),
                    Some(5) => Arc::new(Metal::new(Vec3::new(m.diffuse[0] as f32, m.diffuse[1] as f32, m.diffuse[2] as f32), (1. / m.shininess) as f32)),
                    _ => Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(m.diffuse[0] as f32, m.diffuse[1] as f32, m.diffuse[2] as f32))))
                };

                mat
            }).collect()
        },
        Err(e) => {
            vec![Arc::new(Lambertian::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))))]
        }
    };

    for m in models.iter() {
        let mesh = &m.mesh;

        for f in mesh.indices.chunks(3) {
            let v0 = Vec3::new(mesh.positions[f[0] as usize * 3] as f32, mesh.positions[f[0] as usize * 3 + 1] as f32, mesh.positions[f[0] as usize * 3 + 2] as f32);
            let v1 = Vec3::new(mesh.positions[f[1] as usize * 3] as f32, mesh.positions[f[1] as usize * 3 + 1] as f32, mesh.positions[f[1] as usize * 3 + 2] as f32);
            let v2 = Vec3::new(mesh.positions[f[2] as usize * 3] as f32, mesh.positions[f[2] as usize * 3 + 1] as f32, mesh.positions[f[2] as usize * 3 + 2] as f32);

            let mat = match mesh.material_id {
                Some(id) => Arc::clone(&materials[id]),
                None => Arc::clone(&default_material)
            };

            let tri: Triangle;

            if mesh.normals.len() > 0 {
                let normal = Vec3::new(mesh.normals[f[0] as usize * 3] as f32, mesh.normals[f[1] as usize * 3] as f32, mesh.normals[f[2] as usize * 3] as f32);
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