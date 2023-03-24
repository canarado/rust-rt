use crate::{vec3::Vec3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> ConstantTexture { ConstantTexture { color }}
}

impl Texture for ConstantTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture<T: Texture, Y: Texture> {
    pub odd: T,
    pub even: Y
}

impl<T: Texture, Y: Texture> CheckerTexture<T, Y> {
    pub fn new(odd: T, even: Y) -> CheckerTexture<T, Y> { CheckerTexture { odd, even }}
}

impl<T: Texture, Y: Texture> Texture for CheckerTexture<T, Y> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = f64::sin(10. * p.x) * f64::sin(10. * p.y) * f64::sin(10. * p.z);
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}