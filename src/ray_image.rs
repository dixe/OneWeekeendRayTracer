use crate::types::*;
use image::{RgbImage, ImageBuffer};


pub struct RayImage {
    pub height: usize,
    pub width: usize,
    // pixel data in float
    pub data: Vec<Color>,

}


impl RayImage {

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::default(); width * height],
        }
    }



    pub fn insert_ij_data(&mut self, i: f64, j: f64, color: Color) {


    }



    pub fn save_png(&self, path: &str) {

        let img: RgbImage = ImageBuffer::from_vec(self.width as u32, self.height as u32, self.rbg_image_data()).unwrap();

        img.save(path);

    }

    fn rbg_image_data(&self) -> Vec::<u8> {

        let mut res = Vec::new();
        for pixel_index in 0..(self.width*self.height) {

            let index = pixel_index;

            let color = self.data[index];

            // use sqrt for gamme 2.0
            let r = (255.999 * f64::sqrt(color.x).clamp(0.0, 0.999)) as u8;
            let g = (255.999 * f64::sqrt(color.y).clamp(0.0, 0.999)) as u8;
            let b = (255.999 * f64::sqrt(color.z).clamp(0.0, 0.999)) as u8;

            res.push(r);
            res.push(g);
            res.push(b);

        }
        res
    }
}
