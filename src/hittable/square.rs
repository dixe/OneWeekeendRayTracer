use crate::hittable::*;
use crate::types::*;
use crate::random_utils::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};
use crate::ray::Ray;
use crate::material::*;


pub struct Square {
    center: Point,
    side_half: f64,
    material_id: usize,
}

impl Square {

    pub fn new(center: Point, side_length: f64, material_id: usize) -> Self {
        Self {
            center,
            side_half: side_length/2.0,
            material_id
        }
    }
}

struct Plane {
    normal: Vec3,
    point: Vec3
}


impl HittableFuncs for Square {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let p0 = self.center + Vec3::new(0.0, self.side_half, self.side_half);

        let norm = Vec3::new(0.0, 0.0, 1.0).normalize();


        let face_plane = Plane {
            normal: norm,
            point: p0
        };

        let l = ray.dir();
        let ln_dot = l.dot(&norm);

        if ln_dot == 0.0 {
            // TODO: just this plane does not interect
            return None;
        }

        let d = (p0 - ray.origin()).dot(&norm) / ln_dot;

        let p = ray.at(d);
        // TODO: Check that P is inside the square

        if d < 0.001 {
            return None;
        }

        if p.x < self.center.x + self.side_half &&
            p.x > self.center.x - self.side_half &&
            p.y > self.center.y - self.side_half &&
            p.y < self.center.y + self.side_half {
                if d == 0.0 {

                    panic!("ZERO ZERO D");
                }
                return Some(HitRecord::new(d, p, norm, self.material_id))
            }

        None
    }


}
