use std::fs;
use rand::Rng;

// Mods
mod types;
mod ray_image;
mod ray;
mod camera;
mod hittable;
mod random_utils;
mod material;


// Imports
use types::*;
use ray_image::*;
use ray::{Ray};
use hittable::*;
use material::*;


struct Parameters<'a> {
    samples_per_pixel: u32,
    max_depth: u32,
    materials: Vec::<&'a dyn Material>,
}

fn main() {


    // RayImage

    let aspect_ratio = 16.0/9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;



    // World
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));


    let materials : Vec::<& dyn Material> = vec![&material_ground, &material_center, &material_left, &material_right];


    // TODO: Maybe use shared pointer RC instead of a usize index into array
    let sphere_ground = Sphere::new(Point::new(0.0,  -100.5,  -1.0), 100.0, 0);
    let sphere_center = Sphere::new(Point::new(0.0 ,    0.0,  -1.0),   0.5,   1);
    let sphere_left =   Sphere::new(Point::new(-1.0,    0.0,  -1.0),   0.5,   2);
    let sphere_right =  Sphere::new(Point::new(1.0,     0.0,  -1.0),   0.5,   3);


    let mut world = HittableList::new();
    world.add(&sphere_ground);
    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_right);


    let params = Parameters {
        samples_per_pixel: 100,
        max_depth: 50,
        materials: materials,
    };

    let mut ray_image = ray_image::RayImage::empty(width, height);



    // Camera

    let viewport_height = 2.0;
    let camera = camera::Camera::new(aspect_ratio, viewport_height);


    // Render
    render_ray_image(&mut ray_image, &camera, &world, &params);

    ray_image.save_png("test.png");

}

fn render_ray_image(ray_image: &mut RayImage, camera: &camera::Camera, world: &dyn Hittable, params: &Parameters) {
    let mut rng = rand::thread_rng() ;

    // writte from left to right, top to bottom.
    // Thats why we do .rev on j and use an incrementing index
    // and not a calucalted index = j * width + i
    // That would flip the ray_image in X axis
    let mut index = 0;


    for j in (0..ray_image.height).rev() {
        print!("\rScanLione remaining: {:?} ", j);
        for i in 0..ray_image.width {

            let mut color = Vec3::default();

            for sample in 0..params.samples_per_pixel {

                let u = (i as f64 + rng.gen::<f64>()) / (ray_image.width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (ray_image.height - 1) as f64;


                let ray = camera.uv_ray(u, v);
                color += ray_color(&ray, world, params.max_depth, &params.materials);
            }

            ray_image.data[index] = color / (params.samples_per_pixel as f64);

            index += 1;

        }
    }
    println!("\nDone");
}



fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32, materials: &Vec::<& dyn Material>) -> Color {

    if depth <= 0 {
        return Color::default();
    }

    // if we hit the sphere use that colro
    if let Some(hit) = world.hit(&ray, 0.001, f64::MAX) {

        match materials[hit.material_id()].scatter(ray, &hit) {
            Some(scatter) => {
                // TODO: rename to attenuation
                let c: Color = scatter.color;

                let r_c: Color = ray_color(&scatter.ray, world, depth - 1, &materials);
                return c.mul(&r_c);
            },
            None => {
                return Color::default();
            }
        };
    }

    // not hit, use background color
    let dir = ray.dir();

    let t = 0.5 * (dir.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)

}
