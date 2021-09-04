use nalgebra as na;
use crate::types::*;
use crate::ray::Ray;


mod hittable_list;
pub use self::hittable_list::*;


#[derive(Clone, Copy)]
pub struct HitRecord {
    point: Point,
    // Face normal. Always points outward, in book they always point toward ray, might cause some bugs when just following along
    // Maybe store both, facenormal and ray normal, if we hit from outside they will be the same
    normal: Direction,
    t: f64
}

impl HitRecord {

    pub fn normal(&self) -> Vec3 {
        *self.normal
    }
}


pub trait Hittable {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

}


pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {

    pub fn new(center: Point, radius: f64) -> Self {
        Self {
            center,
            radius
        }
    }
}


impl Hittable for Sphere {


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
        let normal = na::Unit::new_normalize((point - self.center) / self.radius);

        return Some(HitRecord {
            t,
            point,
            normal
        });

    }

}
