extern crate rand;

mod camera;
mod geometry;
mod material;
mod ray;
mod window;

use camera::Camera;
use geometry::sphere::Sphere;
use material::Material;
use ray::Ray;
use window::Window;

use glam::{Vec2, Vec3};
use image::ImageBuffer;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Work {
    lower_left_tile: (u32, u32),
    upper_right_tile: (u32, u32),
    //NOTE(stanisz): rows from top to bottom
    colors: Vec<Vec3>,
}

impl Work {
    pub fn new(lower_left_tile: (u32, u32), upper_right_tile: (u32, u32)) -> Work {
        Work {
            lower_left_tile,
            upper_right_tile,
            colors: Vec::<Vec3>::new(),
        }
    }
}

pub struct Stray {
    //TODO(stanisz): rays_per_pixel
    window: Window,
    camera: Camera,

    //NOTE(staneesh): sphere and its material index
    spheres: Vec<(Sphere, u32)>,
    //NOTE(staneesh): material and its index
    materials: Vec<(Material, u32)>,

    tracing_depth: u32,

    background_color: Vec3,
    rays_per_pixel: u32,

    tile_size: u32,
    number_of_threads: u32,
}

impl Stray {
    pub fn new() -> Stray {
        Stray {
            window: Window::new(1024, 1024),
            camera: Camera::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(1.0, 1.0),
                1.0,
            ),

            spheres: Vec::<(Sphere, u32)>::new(),
            materials: Vec::<(Material, u32)>::new(),

            tracing_depth: 5,
            background_color: Vec3::zero(),
            rays_per_pixel: 4,
            tile_size: 100,
            number_of_threads: 1,
        }
    }

    pub fn set_number_of_threads(&mut self, new_number_of_threads: u32) {
        self.number_of_threads = new_number_of_threads;
    }
    pub fn set_rays_per_pixel(&mut self, new_rays_per_pixel: u32) {
        self.rays_per_pixel = new_rays_per_pixel;
    }

    pub fn set_depth(&mut self, new_tracing_depth: u32) {
        self.tracing_depth = new_tracing_depth;
    }

    pub fn set_window_dimensions(&mut self, width: u32, height: u32) {
        self.window = Window::new(width, height);
        let a_ratio = width as f32 / height as f32;
        self.camera = Camera::new(
            self.camera.position,
            self.camera.direction,
            Vec2::new(a_ratio, 1.0),
            self.camera.canvas_distance,
        );
    }

    pub fn set_background(&mut self, color: (f32, f32, f32)) {
        self.background_color = Vec3::new(
            clamp_float_zero_one(color.0),
            clamp_float_zero_one(color.1),
            clamp_float_zero_one(color.2),
        );
    }

    pub fn add_sphere(
        &mut self,
        position: (f32, f32, f32),
        r: f32,
        mat_index: u32,
    ) -> Result<(), &'static str> {
        let x = position.0;
        let y = position.1;
        let z = position.2;
        let sphere = Sphere::new(Vec3::new(x, y, z), r);
        if self.find_material_by_index(&mat_index).is_none() {
            return Err("No material of such index!");
        }

        self.spheres.push((sphere, mat_index));
        return Ok(());
    }

    pub fn add_material(
        &mut self,
        color: (f32, f32, f32),
        shininess: f32,
        mat_index: u32,
    ) -> Result<(), &'static str> {
        if self.find_material_by_index(&mat_index).is_some() {
            return Err("Cannot add another \
                       material with the same index!");
        }

        let material = Material::new(color, false, shininess);
        self.materials.push((material, mat_index));

        return Ok(());
    }

    pub fn add_emit_material(
        &mut self,
        color: (f32, f32, f32),
        mat_index: u32,
    ) -> Result<(), &'static str> {
        if self.find_material_by_index(&mat_index).is_some() {
            return Err("Cannot add another \
                       material with the same index!");
        }

        let material = Material::new(color, true, 0.0);
        self.materials.push((material, mat_index));

        return Ok(());
    }
    fn find_material_by_index(&self, mat_index: &u32) -> Option<Material> {
        for (_position, (current_material, cur_mat_index)) in self.materials.iter().enumerate() {
            if cur_mat_index == mat_index {
                return Some(*current_material);
            }
        }

        return None;
    }

    fn ray_cast(&self, ray_pass: &Ray) -> Vec3 {
        let mut ray: Ray = *ray_pass;
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        let mut ray_energy = Vec3::new(1.0, 1.0, 1.0);
        let min_hit_dist = 0.001;

        for _bounce_index in 1..self.tracing_depth + 1 {
            //TODO(stanisz): check for correctness
            if ray_energy.length() < 0.00001 {
                break;
            }

            //NOTE(staneesh): should this be in a struct 'hit_record' or sth?
            let mut min_dist_sq_to_sphere = 100000.0;
            let mut closest_sphere = Option::<Sphere>::None;
            let mut closest_hit_point = Option::<Vec3>::None;
            let mut closest_mat_index = Option::<u32>::None;

            for (_index, (test_sphere, mat_index)) in self.spheres.iter().enumerate() {
                if let Some(hit_sphere_point) = ray.hit_sphere(&test_sphere) {
                    let l_sq = (hit_sphere_point - ray.position).length_squared();

                    if l_sq < min_dist_sq_to_sphere && l_sq > min_hit_dist * min_hit_dist {
                        closest_sphere = Some(*test_sphere);
                        min_dist_sq_to_sphere = l_sq;
                        closest_hit_point = Some(hit_sphere_point);
                        closest_mat_index = Some(*mat_index);
                    }
                }
            }

            if closest_sphere.is_some() {
                let test_sphere = closest_sphere.unwrap();
                let hit_sphere_point = closest_hit_point.unwrap();
                let mat_index = closest_mat_index.unwrap();

                let normal_to_sphere_surface =
                    (hit_sphere_point - test_sphere.position).normalize();

                let mut contrib = Vec3::dot(-ray.direction.normalize(), normal_to_sphere_surface);

                contrib = clamp_float_zero_one(contrib);

                let material_hit = self.find_material_by_index(&mat_index).unwrap();

                //println!("{}",diffuse_lighting);
                if material_hit.is_a_source {
                    color = color + hadamard(&ray_energy, &material_hit.color);
                    ray_energy = Vec3::zero();
                } else {
                    ray_energy = hadamard(&ray_energy, &(material_hit.color * contrib));
                }

                let prefect_reflection = ray.direction + 2.0 * (contrib) * normal_to_sphere_surface;

                let new_dir = lerp::<Vec3>(
                    (random_in_unit_sphere() + prefect_reflection.normalize()).normalize(),
                    prefect_reflection,
                    material_hit.shininess,
                );

                let new_ray = Ray::new(hit_sphere_point, new_dir);
                ray = new_ray;
            } else {
                //NOTE(stanisz): hit background

                color += hadamard(&ray_energy, &self.background_color);
                return color;
            }
        }
        return color;
    }

    fn fill_work_queue(&self) -> Vec<Work> {
        let mut work_queue = Vec::<Work>::new();

        let number_of_tiles_x = self.window.width / self.tile_size;
        let number_of_tiles_y = self.window.height / self.tile_size;

        for y in 0..number_of_tiles_y {
            let y_coord = y * self.tile_size;
            let y_coord_next = y_coord + self.tile_size;

            for x in 0..number_of_tiles_x {
                let x_coord = x * self.tile_size;
                let x_coord_next = x_coord + self.tile_size;

                work_queue.push(Work::new((x_coord, y_coord), (x_coord_next, y_coord_next)));
            }
        }
        for x in 1..(self.window.width / self.tile_size) as u32 + 1 {
            let h_lower = (self.window.height / self.tile_size) as u32 * self.tile_size;

            work_queue.push(Work::new(
                ((x - 1) * self.tile_size, h_lower),
                (x * self.tile_size, self.window.height),
            ));
        }
        for y in 1..(self.window.height / self.tile_size) as u32 + 1 {
            let x_lower = (self.window.width / self.tile_size) as u32 * self.tile_size;

            work_queue.push(Work::new(
                (x_lower, (y - 1) * self.tile_size),
                (self.window.width, y * self.tile_size),
            ));
        }

        work_queue.push(Work::new(
            (
                (self.window.width / self.tile_size) as u32 * self.tile_size,
                (self.window.height / self.tile_size) as u32 * self.tile_size,
            ),
            (self.window.width, self.window.height),
        ));

        return work_queue;
    }

    fn worker_job(&self, work_queue: Arc<Mutex<&mut Vec<Work>>>, work_index: Arc<Mutex<u32>>) {
        let mut cur_index = work_index.lock().unwrap();
        let work_task = work_queue.lock().unwrap()[*cur_index as usize].clone();

        let task_width = work_task.upper_right_tile.0 - work_task.lower_left_tile.0;
        let task_height = work_task.upper_right_tile.1 - work_task.lower_left_tile.1;

        for y in 0..task_height {
            for x in 0..task_width {
                let u = x as f32 / (task_width - 1) as f32;
                let v = y as f32 / (task_height - 1) as f32;

                let new_x = lerp(
                    work_task.lower_left_tile.0 as f32,
                    work_task.upper_right_tile.0 as f32,
                    u,
                );
                let new_y = lerp(
                    work_task.lower_left_tile.1 as f32,
                    work_task.upper_right_tile.1 as f32,
                    v,
                );

                let camera_x = lerp(
                    self.camera.get_lower_left_canvas().x(),
                    self.camera.get_upper_right_canvas().x(),
                    new_x / self.window.width as f32,
                );

                let camera_y = lerp(
                    self.camera.get_lower_left_canvas().y(),
                    self.camera.get_upper_right_canvas().y(),
                    new_y / self.window.height as f32,
                );
                let new_z = self.camera.get_lower_left_canvas().z();

                let current_pixel = Vec3::new(camera_x, camera_y, new_z);
                let current_ray = Ray::new(
                    self.camera.position,
                    (current_pixel - self.camera.position).normalize(),
                );

                let mut pixel_color = Vec3::zero();

                for _cur_ray_per_pixel in 0..self.rays_per_pixel {
                    pixel_color += self.ray_cast(&current_ray) / (self.rays_per_pixel as f32);
                }

                pixel_color = clamp_vec3_zero_one(&pixel_color);
                //TODO(stanisz): should this correction happen for colors in range [0;255]?
                pixel_color = gamma_correct_color(&pixel_color);

                work_queue.lock().unwrap()[*cur_index as usize]
                    .colors
                    .resize((task_width * task_height) as usize, Vec3::zero());
                work_queue.lock().unwrap()[*cur_index as usize].colors
                    [(y * task_width + x) as usize] = pixel_color;

                *cur_index += 1;
            }
        }
    }
    pub fn render_scence(&self) {
        let raw_queue = &mut self.fill_work_queue();
        let work_queue = Arc::new(Mutex::from(raw_queue));
        let work_queue_len = work_queue.lock().unwrap().len();

        let work_task_index = Arc::new(Mutex::new(0));

        loop {
            self.worker_job(Arc::clone(&work_queue), Arc::clone(&work_task_index));
            if *work_task_index.lock().unwrap() == work_queue_len as u32 {
                break;
            }
        }

        //NOTE(stanisz): join threads, now i want to singlethreadingly
        // fill the ImageBuffer with the data computed asynchronously.
        // The data is stored in the work_queue.

        let mut img = ImageBuffer::new(self.window.width, self.window.height);

        for i in 0..work_queue_len {
            let work_task = &work_queue.lock().unwrap()[i as usize];
            let task_height = work_task.upper_right_tile.1 - work_task.lower_left_tile.1;
            let task_width = work_task.upper_right_tile.0 - work_task.lower_left_tile.0;

            for y in 0..task_height {
                let img_y = work_task.lower_left_tile.1 + y;
                for x in 0..task_width {
                    let img_x = work_task.lower_left_tile.0 + x;

                    let color_vec = work_task.colors[(y * task_width + x) as usize];

                    let r = (color_vec.x() * 255.0) as u8;
                    let g = (color_vec.y() * 255.0) as u8;
                    let b = (color_vec.z() * 255.0) as u8;

                    //println!("color: {} {} {}", r, g, b);

                    let pixel = image::Rgb([r, g, b]);
                    img.put_pixel(img_x, img_y, pixel);
                }
            }
        }

        img.save("RustyBeautyAsyncPrepare.png").unwrap();
    }
}

fn color_to_mask(v: &Vec3) -> (u8, u8, u8) {
    (
        (v.x() * 255.0) as u8,
        (v.y() * 255.0) as u8,
        (v.z() * 255.0) as u8,
    )
}

fn gamma_correct_color(v: &Vec3) -> Vec3 {
    Vec3::new(v.x().sqrt(), v.y().sqrt(), v.z().sqrt())
}

fn clamp_float_zero_one(a: f32) -> f32 {
    let mut res = a;
    if res < 0.0 {
        res = 0.0;
    } else if res > 1.0 {
        res = 1.0;
    }

    return res;
}

fn clamp_vec3_zero_one(v: &Vec3) -> Vec3 {
    Vec3::new(
        clamp_float_zero_one(v.x()),
        clamp_float_zero_one(v.y()),
        clamp_float_zero_one(v.z()),
    )
}

fn lerp<T>(a: T, b: T, t: f32) -> T
where
    T: std::ops::Mul<f32, Output = T> + std::ops::Add<T, Output = T>,
{
    a * (1.0 - t) + b * t
}

fn hadamard(a: &Vec3, b: &Vec3) -> Vec3 {
    let result = Vec3::new(a.x() * b.x(), a.y() * b.y(), a.z() * b.z());

    return result;
}

fn random0_1() -> f32 {
    let mut rng = rand::thread_rng();
    let val: f32 = rng.gen();

    return val;
}

fn random_range(a: f32, b: f32) -> f32 {
    return random0_1() * (b - a) + a;
}

//TODO(staneesh): read about this
fn random_in_unit_sphere() -> Vec3 {
    let a = random_range(0.0, 2.0 * 3.1415);
    let z = random_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}
