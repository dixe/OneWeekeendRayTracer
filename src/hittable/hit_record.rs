use crate::types::*;
use crate::random_utils::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};
use crate::ray::Ray;
use nalgebra as na;
use crate::material::*;


pub struct HitRecord {
    point: Point,
    // Face normal. Always points outward, in book they always point toward ray, might cause some bugs when just following along
    // Maybe store both, facenormal and ray normal, if we hit from outside they will be the same
    normal: Direction,
    t: f64,
    material_id: usize
}

impl HitRecord {

    pub fn new(t: f64, point: Point, normal: Vec3, material_id: usize) -> Self {
        Self {
            t,
            point,
            normal: na::Unit::new_normalize(normal),
            material_id
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        *self.normal
    }

    pub fn material_id(&self) -> usize {
        self.material_id
    }

    pub fn random_diffuse_ray(&self) -> Ray {

        // TODO: make sure the normal is on the same side as the ray

        // Diffuse 1
        //let target = *self.normal + random_unit_vector();
        //Diffuse 2
        //let target = *self.normal + random_in_unit_sphere();
        // Diffuse 3
        let direction = *self.normal + random_in_hemisphere(&self.normal);


        Ray::new(self.point, direction)
    }

    pub fn random_scatter_ray(&self) -> Ray {

        let mut scatter_direction = *self.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if (scatter_direction.near_zero()){
            scatter_direction = *self.normal;
        }

        Ray::new(self.point, scatter_direction)
    }

    pub fn reflected_ray(&self, ray_in: &Ray) -> Option<Ray> {
        let reflected = ray_in.unit_dir().reflect(&self.normal);

        return Some(Ray::new(self.point, reflected))




    }
}
