use crate::types::*;
use image::{RgbImage, ImageBuffer};


pub struct RayImage {
    pub height: usize,
    pub width: usize,
    pub samples_per_pixel: usize,
    // pixel data in float
    pub data: Vec<Color>,

}


impl RayImage {

    pub fn empty(width: usize, height: usize, samples_per_pixel: usize) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            data: vec![Color::new(0.0, 0.0, 0.0); width * height * samples_per_pixel],
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

            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for sample in 0..self.samples_per_pixel {

                let index = pixel_index * self.samples_per_pixel + sample;
                color += self.data[index];
            }

            color = color / (self.samples_per_pixel as f64);

            let r = (255.999 * color.x) as u8;
            let g = (255.999 * color.y) as u8;
            let b = (255.999 * color.z) as u8;

            res.push(r);
            res.push(g);
            res.push(b);

        }
        res
    }

    /*
    pub fn ppm_string(&self) -> String{

    // TODO: Maybe scale data to 0-1 before doing the output
    let mut output = format!("P3\n{} {}\n255\n", self.width, self.height);


    println!("We have {:?} pixels, we expect {}", self.data.len(), self.width * self.height);
    let mut rows = 0;
    let mut pixels  = 0;

    // TODO: maybe use rbg_image_data method
    for pixel in &self.data {

    pixels += 3;
    let r = (255.999 * pixel.x) as i32;
    let g = (255.999 * pixel.y) as i32;
    let b = (255.999 * pixel.z) as i32;
    output += format!("{} {} {}\n", r, g, b).as_str();

}

    println!("rows: {:?}\nPixels: {}", rows, pixels);

    output.to_string()
}
     */
}
