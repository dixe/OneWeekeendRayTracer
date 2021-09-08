use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::types::*;


mod lambertian;

mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub color: Color
}

pub  trait MaterialFuncs {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}


#[derive(Copy,Clone)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal)
}

impl Material {

    pub fn lambertian(albedo: Color) -> Self {
        Material::Lambertian(lambertian::Lambertian::new(albedo))
    }

    pub fn metal(albedo: Color) -> Self {
        Material::Metal(metal::Metal::new(albedo))
    }
}


impl MaterialFuncs for Material {

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, hit_record),
            Material::Metal(m) =>  m.scatter(ray_in, hit_record)
        }
    }
}
