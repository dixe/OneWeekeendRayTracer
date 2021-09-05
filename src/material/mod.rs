use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::types::*;


mod lambertian;
pub use self::lambertian::*;



mod metal;
pub use self::metal::*;


pub struct Scatter {
    ray: Ray,
    color: Color
}

pub trait Material {

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;


}
