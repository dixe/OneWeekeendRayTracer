use crate::types::*;

use nalgebra as na;

pub struct Ray {
    origin: Point,
    dir: Direction, // Maybe make this a unit vector3
}



impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Self {
            origin,
            dir: na::Unit::new_normalize(dir)
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        *self.dir
    }


    pub fn at(&self, t: f64) -> Point {
        self.origin + t * (*self.dir)
    }

}
