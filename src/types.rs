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


pub trait NearZero {
    fn near_zero(&self) -> bool;
}

impl NearZero for na::Vector3::<f64> {
    fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
}


pub trait Reflection {
    fn reflect(&self, normal: &na::Unit::<Vec3>) -> Vec3;
}

impl Reflection for na::Unit<na::Vector3::<f64>> {

    fn reflect(&self, normal: &na::Unit::<Vec3>) -> Vec3 {
        let n = **normal;
        let v = **self;
        // THis is - in the online book, but our normals points out not in
        v + 2.0 * n.dot(&n) * n

    }

}


pub trait PairMul {
    fn mul(&self, other: &Self) -> Self;
}

impl PairMul for na::Vector3::<f64> {
    fn mul(&self, other: &Self) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

}
