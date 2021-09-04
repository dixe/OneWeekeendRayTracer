use std::fs;
use rand::Rng;

mod types;
mod ray_image;
mod ray;
mod camera;
mod hittable;


use types::*;
use ray_image::*;
use ray::{Ray};
use hittable::*;

fn main() {


    // RayImage

    let aspect_ratio = 16.0/9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;


    let samples_per_pixel = 100;

    let mut ray_image = ray_image::RayImage::empty(width, height, samples_per_pixel);


    // World
    let sphere_origin = Point::new(0.0, 0.0, -1.0);
    let sphere_1 = Sphere::new(sphere_origin, 0.5);
    let sphere_2 = Sphere::new(Point::new(0.0,-100.5,-1.0), 100.0);

    let mut world = HittableList::new();
    world.add(&sphere_1);
    world.add(&sphere_2);


    // Camera

    let viewport_height = 2.0;
    let camera = camera::Camera::new(aspect_ratio, viewport_height);


    // Render
    render_ray_image(&mut ray_image, &camera, &world);

    ray_image.save_png("test.png");

}

fn render_ray_image(ray_image: &mut RayImage, camera: &camera::Camera, world: &dyn Hittable) {
    let mut rng = rand::thread_rng() ;

    let ray = camera.uv_ray(0.0, 0.0);
    let hit = ray_color(&ray, world);

    println!("{:?}", hit);


    // writte from left to right, top to bottom.
    // Thats why we do .rev on j and use an incrementing index
    // and not a calucalted index = j * width + i
    // That would flip the ray_image in X axis
    let mut index = 0;


    for j in (0..ray_image.height).rev() {
        print!("\rScanLione remaining: {:?} ", j);
        for i in 0..ray_image.width {

            let mut color = Vec3::default();

            // TODO: Maybe don't store samples_per_pixel in ray_image. But in a general settings.
            // And do the division here before we add to image data

            for sample in 0..ray_image.samples_per_pixel {

                let u = (i as f64 + rng.gen::<f64>())  / (ray_image.width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (ray_image.height - 1) as f64;


                let ray = camera.uv_ray(u, v);
                color += ray_color(&ray, world);
            }
            ray_image.data[index] = color;

            index += 1;

        }
    }
    println!("\nDone");
}



fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {

    // if we hit the sphere use that color


    match world.hit(&ray, 0.0, f64::MAX) {
        None => {},
        Some(record) => {
            return 0.5 * (record.normal() + Vec3::new(1.0, 1.0, 1.0));
        }
    }


    // not hit, use background color
    let dir = ray.dir();

    let t = 0.5 * (dir.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)

}
