use crate::hittable::*;
use crate::types::*;
use crate::random_utils::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};
use crate::ray::Ray;
use crate::material::*;

pub struct Sphere {
    center: Point,
    radius: f64,
    material_id: usize,
}

impl Sphere {

    pub fn new(center: Point, radius: f64, material_id: usize) -> Self {
        Self {
            center,
            radius,
            material_id
        }
    }
}


impl HittableFuncs for Sphere {


    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let ray_dir = ray.dir();

        let a = ray_dir.length_squared();
        let half_b = oc.dot(&ray_dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }


        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        return Some(HitRecord::new(t, point, normal, self.material_id));

    }

}
