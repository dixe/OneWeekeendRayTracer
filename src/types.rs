use nalgebra as na;


pub type Color = na::Vector3::<f64>;
pub type Point = na::Vector3::<f64>;

pub type Vec3 = na::Vector3::<f64>;


pub type Direction = na::UnitVector3::<f64>;




pub trait SquareLength {

    fn length_squared(&self) -> f64;
}


impl SquareLength for na::Vector3::<f64> {

    fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z

    }
}
