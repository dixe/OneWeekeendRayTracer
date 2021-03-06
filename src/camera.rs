use crate::types::*;
use crate::ray;

#[derive(Clone)]
pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}


impl Camera {

    pub fn new(aspect_ratio: f64, viewport_height: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point::default();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let focal_length = 1.0;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn uv_ray(&self, u: f64, v: f64) -> ray::Ray {

        ray::Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)

    }
}
