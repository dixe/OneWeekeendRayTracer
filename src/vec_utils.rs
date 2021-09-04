use crate::types::*;
use rand::Rng;

pub fn random(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(min..max);
    let y = rng.gen_range(min..max);
    let z = rng.gen_range(min..max);
    Vec3::new(x, y,z)
}


pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    while true {

        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let z = rng.gen_range(-1.0..1.0);
        let v = Vec3::new(x, y,z);

        if v.length_squared() < 1.0 {
            return v;
        }
    }
    panic!("Exited loop without return");
}
