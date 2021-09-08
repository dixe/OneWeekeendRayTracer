use crate::hittable::*;

pub struct HittableList {
    objects: Vec::<Hittable>
}

impl HittableList {

    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

}


impl HittableFuncs for HittableList {

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
                        Some(ref current_hit) => {
                            if new_hit.t() < current_hit.t() {
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
