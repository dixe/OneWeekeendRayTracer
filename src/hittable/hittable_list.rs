use crate::hittable::*;

pub struct HittableList<'a> {

    objects: Vec::<&'a dyn Hittable>

}

impl<'a> HittableList<'a> {

    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }

}


impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        let mut current_hit_record= None;

        for object in &self.objects {
            match object.hit(ray, t_min, t_max) {
                Some(new_hit) => {
                    // Update hit record to take the closest
                    match current_hit_record {
                        None => {
                            current_hit_record = Some(new_hit);
                        },
                        Some(current_hit) => {
                            if new_hit.t < current_hit.t {
                                current_hit_record = Some(new_hit);
                            }

                        },
                    };
                },
                None => {}
            };

        }


        current_hit_record
    }
}
