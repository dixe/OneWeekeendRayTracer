use crate::types::*;
use crate::material::*;


#[derive(Copy,Clone)]
pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}


impl MaterialFuncs for Lambertian {

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {

        let scatter_ray = hit_record.random_scatter_ray();

        Some(Scatter {
            ray: scatter_ray,
            color: self.albedo
        })

    }

}
