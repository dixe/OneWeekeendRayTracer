use crate::types::*;
use crate::material::*;



pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}


impl Material for Metal {

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {

        match hit_record.reflected_ray(ray_in) {
            None => None,
            Some(scatter_ray) =>{
                Some(Scatter {
                    ray: scatter_ray,
                    color: self.albedo
                })
            }
        }

    }

}
