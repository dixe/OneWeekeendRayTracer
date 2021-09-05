use std::fs;
use rand::Rng;
use std::time::Instant;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use rand::seq::SliceRandom;

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
    height: usize,
    width: usize,
    extra_threads: u32
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
    let sphere_center = Sphere::new(Point::new(0.0 ,    0.0,  -1.5),   0.5,   1);
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
        width,
        height,
        extra_threads: 30
    };

    let mut ray_image = ray_image::RayImage::empty(width, height);


    // Camera

    let viewport_height = 2.0;
    let camera = camera::Camera::new(aspect_ratio, viewport_height);


    // Render

    let now = Instant::now();
    //render_ray_image_master_slave(&mut ray_image, &camera, &world, &params);
    render_ray_image_random_distributed(&mut ray_image, &camera, &world, &params);

    let elapsed = now.elapsed();

    println!("It took {:?}", elapsed.as_secs());

    ray_image.save_png("test.png");

}


#[derive(Clone, Copy)]
struct Work {
    i: usize,
    j: usize,
    index: usize
}


#[derive(Clone, Copy)]
struct ThreadResult {
    color: Color,
    index: usize
}



#[derive(Clone, Copy)]
enum SlaveMessage {
    RequestWork,
    WorkDone(ThreadResult),
}

#[derive(Clone, Copy)]
enum MasterMessage {
    DoWork(Work),
    Done
}

struct Worker {
    mosi: Sender<MasterMessage>,
    miso: Receiver<SlaveMessage>,
    handle: thread::JoinHandle<()>,
    done: bool
}

#[derive(Clone, Copy)]
struct ThreadWorkingData {
    params: usize, // &'a Parameters<'a>,
    camera: usize, // &'a camera::aCamera,
    world: usize
}



fn render_ray_image_random_distributed(ray_image: &mut RayImage, camera: &camera::Camera, world: &HittableList, params: &Parameters){
    let mut rng = rand::thread_rng() ;

    let mut index = 0;
    //make list of work
    let mut work_tasks = Vec::new();
    for j in (0..ray_image.height).rev() {

        for i in 0..ray_image.width {
            work_tasks.push(Work {i,j, index});
            index += 1;
        }
    }

    work_tasks.shuffle(&mut rng);



    let thread_work_count = work_tasks.len() / (params.extra_threads + 1) as usize;

    println!("Each thread should to {:?} total work is {}", thread_work_count, work_tasks.len());

    let work_data = ThreadWorkingData {
        camera: (camera as *const camera::Camera) as usize,
        params: (params as *const Parameters) as usize,
        world: (world as *const HittableList ) as usize
    };




    let mut children = Vec::new();
    let mut work_index = 0;
    for id in 0..params.extra_threads {

        let mut thread_tasks = Vec::new();
        for i in work_index..(thread_work_count + work_index) {
            thread_tasks.push(work_tasks[i]);
        }

        work_index += thread_work_count;
        let t_data = work_data.clone();

        let child = thread::spawn(move || {

            let world;
            let camera;
            let params;
            unsafe {
                world = & *(t_data.world as * const HittableList);
                params = & *(t_data.params as * const Parameters);
                camera = & *(t_data.camera as * const camera::Camera);
            }

            let mut res = vec![ThreadResult { color:Color::default(), index: 0 }; thread_tasks.len() ];

            for index in 0..thread_tasks.len() {
                let work = thread_tasks[index];
                let color = caclulate_pixel_color(work.i, work.j, camera, world, params);


                res[index].color = color;
                res[index].index = work.index;
            }
            res

        });

        children.push(child);
    }


    // to over last work and then join with children to make the final image
    for index in work_index..work_tasks.len() {
        let work = work_tasks[index];
        ray_image.data[work.index] = caclulate_pixel_color(work.i, work.j, camera, world, params);

    }



    for child in children {

        for pixel_res in child.join().expect("joining child failed") {
            ray_image.data[pixel_res.index] = pixel_res.color;
        }
    }


}

fn render_ray_image_master_slave(ray_image: &mut RayImage, camera: &camera::Camera, world: &HittableList, params: &Parameters) {
    // writte from left to right, top to bottom.
    // Thats why we do .rev on j and use an incrementing index
    // and not a calucalted index = j * width + i
    // That would flip the ray_image in X axis

    let mut index = 0;
    //make list of work
    let mut work_tasks = Vec::new();
    for j in (0..ray_image.height).rev() {

        for i in 0..ray_image.width {
            work_tasks.push(Work {i,j, index});
            index += 1;
        }
    }

    println!("h {} w {:?} len {}", params.height, params.width, work_tasks.len());


    let mut children = spawn_threads(camera, world, params);

    let mut work_index = 0;
    let mut work_left = true;

    println!("");

    while work_left {

        let mut all_done = true;
        for worker in children.iter_mut() {
            if worker.done {
                continue;
            }

            match worker.miso.try_recv() {
                Ok(msg) => {
                    match msg {
                        SlaveMessage::RequestWork => {
                            if work_index < work_tasks.len() {
                                worker.mosi.send(MasterMessage::DoWork(work_tasks[work_index]));
                                work_index += 1;

                            }
                            else {
                                worker.mosi.send(MasterMessage::Done);
                            }
                        },
                        SlaveMessage::WorkDone(t_res) => {
                            ray_image.data[t_res.index] = t_res.color;
                        }

                    };

                },
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    worker.done = true;
                }
            };

            all_done &= worker.done;
        }

        work_left = !all_done;
    }
}

fn spawn_threads(camera: &camera::Camera, world: &HittableList, params: &Parameters) -> Vec::<Worker> {
    let mut children = Vec::new();

    let work_data = ThreadWorkingData {
        camera: (camera as *const camera::Camera) as usize,
        params: (params as *const Parameters) as usize,
        world: (world as *const HittableList ) as usize
    };


    for id in 0..params.extra_threads {

        let (miso_sender, miso_rec): (Sender<SlaveMessage>, Receiver<SlaveMessage>) = mpsc::channel();

        let (mosi_sender, mosi_rec): (Sender<MasterMessage>, Receiver<MasterMessage>) = mpsc::channel();


        let t_data = work_data.clone();
        let child = thread::spawn(move || {
            let mut run = true;

            while run {
                miso_sender.send(SlaveMessage::RequestWork);
                let response = mosi_rec.recv();

                match response {
                    Ok(resp) => {

                        match resp {
                            MasterMessage::Done => { run = false;},
                            MasterMessage::DoWork(work) => {
                                // to the work of ray and send result back

                                let world;
                                let camera;
                                let params;
                                unsafe {
                                    world = & *(t_data.world as * const HittableList);
                                    params = & *(t_data.params as * const Parameters);
                                    camera = & *(t_data.camera as * const camera::Camera);
                                }

                                let color = caclulate_pixel_color(work.i, work.j, camera, world, params);


                                let res = ThreadResult {
                                    color,
                                    index: work.index
                                };
                                miso_sender.send(SlaveMessage::WorkDone(res));
                            }
                        };
                    },
                    Err(_) => {run = false}
                }
            }
        });

        children.push(Worker{ handle: child, miso: miso_rec, mosi: mosi_sender, done: false});

    }

    children
}


fn caclulate_pixel_color(i: usize, j: usize, camera: &camera::Camera, world: &dyn Hittable, params: &Parameters) -> Color {
    let mut color = Vec3::default();
    let mut rng = rand::thread_rng() ;

    for sample in 0..params.samples_per_pixel {

        let u = (i as f64 + rng.gen::<f64>()) / (params.width - 1) as f64;
        let v = (j as f64 + rng.gen::<f64>()) / (params.height - 1) as f64;


        let ray = camera.uv_ray(u, v);
        color += ray_color(&ray, world, params.max_depth, &params.materials);
    }

    color / (params.samples_per_pixel as f64)

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
