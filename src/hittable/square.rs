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



    fn face_plane_hit(&self, ray: &Ray, face_plane: &FacePlane) -> Option<HitRecord> {

        let l = ray.dir();
        let ln_dot = l.dot(&face_plane.norm);

        if ln_dot == 0.0 {
            return None;
        }

        let d = (face_plane.p0 - ray.origin()).dot(&face_plane.norm) / ln_dot;

        let p = ray.at(d);

        // TODO: This is kinda wrong. we might hit face, but the inside.
        // But for boxes we should not be inside at any point
        if ln_dot > 0.0 {
            return None;
        }

        if d < 0.001 {
            return None;
        }

        if p.x <= self.center.x + self.side_half &&
            p.x >= self.center.x - self.side_half &&
            p.y >= self.center.y - self.side_half &&
            p.y <= self.center.y + self.side_half &&
            p.z >= self.center.z - self.side_half &&
            p.z <= self.center.z + self.side_half {
                if d == 0.0 {

                    panic!("ZERO ZERO D");
                }
                return Some(HitRecord::new(d, p, face_plane.norm, self.material_id))
            }

        None
    }

}

struct FacePlane {
    norm: Vec3,
    p0: Vec3
}


impl HittableFuncs for Square {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        // FRONT AND BACK
        let p0 = self.center + Vec3::new(0.0, 0.0, self.side_half);
        let norm = Vec3::new(0.0, 0.0, 1.0).normalize();
        let face_front = FacePlane {
            norm,
            p0
        };
        let p0 = self.center - Vec3::new(0.0, 0.0, self.side_half);
        let norm = Vec3::new(0.0, 0.0, -1.0).normalize();
        let face_back = FacePlane {
            norm,
            p0
        };


        // LEFT AND RIGHT
        let p0 = self.center + Vec3::new(self.side_half, 0.0, 0.0);
        let norm = Vec3::new(1.0, 0.0, 0.0).normalize();
        let face_right = FacePlane {
            norm,
            p0
        };
        let p0 = self.center - Vec3::new(self.side_half, 0.0, 0.0);
        let norm = Vec3::new(-1.0, 0.0, 0.0).normalize();
        let face_left = FacePlane {
            norm,
            p0
        };



        // TOP AND BOTTOM
        let p0 = self.center + Vec3::new(0.0, self.side_half, 0.0);
        let norm = Vec3::new(0.0, 1.0, 0.0).normalize();
        let face_top = FacePlane {
            norm,
            p0
        };
        let p0 = self.center - Vec3::new(0.0, self.side_half, 0.0);
        let norm = Vec3::new(0.0, -1.0, 0.0).normalize();
        let face_bottom = FacePlane {
            norm,
            p0
        };


        let right_hit = self.face_plane_hit(ray, &face_right);
        let left_hit = self.face_plane_hit(ray, &face_left);

        let front_hit = self.face_plane_hit(ray, &face_front);
        let back_hit = self.face_plane_hit(ray, &face_back);

        let top_hit = self.face_plane_hit(ray, &face_top);
        let bottom_hit = self.face_plane_hit(ray, &face_bottom);


        let mut hits = 0;

        let mut ret = None;

        if top_hit.is_some() {
            hits += 1;
            ret = top_hit;
        }
        if right_hit.is_some() {
            hits += 1;
            ret = right_hit;
        }
        if front_hit.is_some() {
            hits += 1;
            ret = front_hit;
        }


        if bottom_hit.is_some() {
            hits += 1;
            ret = bottom_hit;
        }
        if left_hit.is_some() {
            hits += 1;
            ret = left_hit;
        }
        if back_hit.is_some() {
            hits += 1;
            ret = back_hit;
        }


        if hits > 1 {
            panic!("mUltiple planes hit");
        }

        ret

    }
}
