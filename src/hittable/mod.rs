use crate::ray::Ray;

mod hittable_list;
pub use self::hittable_list::*;

mod hit_record;
pub use self::hit_record::*;


mod sphere;
pub use self::sphere::*;


pub trait HittableFuncs {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub enum Hittable {
    Sphere(Sphere)
}

impl HittableFuncs for Hittable {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(s) => s.hit(ray, t_min, t_max)
        }
    }
}
