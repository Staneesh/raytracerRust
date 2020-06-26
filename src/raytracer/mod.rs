extern crate rand;

mod geometry;
mod material;
mod camera;
mod ray;
mod window;

use window::Window;
use camera::Camera;
use ray::Ray;
use material::Material;
use geometry::sphere::Sphere;

use glam::{Vec2, Vec3};
use image::{ImageBuffer};
use rand::prelude::*;

pub struct Stray
{
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
}


impl Stray
{
    pub fn new() -> Stray
    {
        Stray
        {
            window: Window::new(1024, 1024),
            camera: Camera::new(
                Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(1.0, 1.0), 1.0
                ),

            spheres: Vec::<(Sphere, u32)>::new(),
            materials: Vec::<(Material, u32)>::new(),

            tracing_depth: 5,
            background_color: Vec3::zero(),
            rays_per_pixel: 4,
        }
    }
    pub fn set_window_dimensions(&mut self, width: u32, height: u32)
    {
        self.window = Window::new(width, height);
        let a_ratio = width as f32 / height as f32;
        self.camera = Camera::new(self.camera.position,
                                  self.camera.direction,
                                  Vec2::new(a_ratio, 1.0),
                                  self.camera.canvas_distance);
    }

    pub fn set_background(&mut self, color: (f32, f32, f32))
    {
        self.background_color = Vec3::new(color.0, color.1, color.2);
    }
    
    pub fn add_sphere(&mut self, position: (f32, f32, f32), r: f32,
                      mat_index: u32)
        -> Result<(), &'static str>
    {
        let x = position.0;
        let y = position.1;
        let z = position.2;
        let sphere = Sphere::new(Vec3::new(x, y, z), r);
        if self.find_material_by_index(&mat_index).is_none()
        {
            return Err("No material of such index!");
        }
        
        self.spheres.push((sphere, mat_index));
        return Ok(());
    }

    pub fn add_material(&mut self,
                        color: (f32, f32, f32),
                        shininess: f32, mat_index: u32)
        -> Result<(), &'static str>
    {
        if self.find_material_by_index(&mat_index).is_some()
        {
            return Err("Cannot add another \
                       material with the same index!");
        }
        
        let material = Material::new(color,
                                     false,
                                     shininess);
        self.materials.push((material, mat_index));

        return Ok(());
    }
   
    pub fn add_emit_material(&mut self,
                        color: (f32, f32, f32),
                        mat_index: u32)
        -> Result<(), &'static str>
    {
        if self.find_material_by_index(&mat_index).is_some()
        {
            return Err("Cannot add another \
                       material with the same index!");
        }
        
        let material = Material::new(color,
                                     true,
                                     0.0);
        self.materials.push((material, mat_index));

        return Ok(());
    }
    fn find_material_by_index(&self, mat_index: &u32) -> Option<Material>
    {
        for (_position, (current_material, cur_mat_index)) in
            self.materials.iter().enumerate()
        {
            if cur_mat_index == mat_index
            {
                return Some(*current_material);
            }
        }

        return None;
    }
    
    fn ray_cast(&self, ray_pass: &Ray) -> Vec3
    {
        let mut ray: Ray = *ray_pass;
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        let mut ray_energy = Vec3::new(1.0, 1.0, 1.0);
        let min_hit_dist = 0.001;

        for _bounce_index in 1..self.tracing_depth + 1
        {
            //TODO(stanisz): check for correctness
            if ray_energy.length() < 0.00001
            {
                break;
            }

            //NOTE(staneesh): should this be in a struct 'hit_record' or sth? 
            let mut min_dist_sq_to_sphere = 100000.0;
            let mut closest_sphere = Option::<Sphere>::None;
            let mut closest_hit_point = Option::<Vec3>::None;
            let mut closest_mat_index = Option::<u32>::None;

            for (_index, (test_sphere, mat_index)) 
                in self.spheres.iter().enumerate()
            {
                if let Some(hit_sphere_point) = ray.hit_sphere(&test_sphere)
                {
                    let l_sq = (hit_sphere_point - ray.position).length_squared();

                    if l_sq < min_dist_sq_to_sphere 
                        && l_sq.sqrt() > min_hit_dist
                    {
                        closest_sphere = Some(*test_sphere);
                        min_dist_sq_to_sphere = l_sq;
                        closest_hit_point = Some(hit_sphere_point); 
                        closest_mat_index = Some(*mat_index);
                    }
                }
            }
            
            if closest_sphere.is_some()
            {
                let test_sphere = closest_sphere.unwrap();
                let hit_sphere_point = closest_hit_point.unwrap();
                let mat_index = closest_mat_index.unwrap();

    
                let normal_to_sphere_surface = (hit_sphere_point - 
                    test_sphere.position).normalize();

                let mut contrib =  Vec3::dot(
                    -ray.direction.normalize(),
                    normal_to_sphere_surface
                        );

                if contrib < 0.0
                {
                    contrib = 0.0;
                }

                let material_hit = 
                    self.find_material_by_index(&mat_index).unwrap();


                //println!("{}",diffuse_lighting);
                if material_hit.is_a_source
                {
                    color = color + hadamard(&ray_energy,
                                             &material_hit.color); 
                    ray_energy = Vec3::zero();
                }
                else
                {
                    ray_energy = hadamard(&ray_energy, 
                                          &(material_hit.color 
                                            * contrib));
                }

                let prefect_reflection = ray.direction + 
                    2.0*(contrib) * normal_to_sphere_surface;

                let new_dir = lerp::<Vec3>(
                    (random_in_unit_sphere() + 
                     normal_to_sphere_surface).normalize(), prefect_reflection,
                    material_hit.shininess);

                let new_ray = Ray::new(hit_sphere_point, new_dir);
                ray = new_ray;
            }
            else 
            {
                //NOTE(stanisz): hit background

                color += hadamard(&ray_energy, &self.background_color);
                return color;
            }
        }
        return color; 
    }

    pub fn render_scence(&self) 
    {
        let debug_display_scanlines_multiple = 16;
        let mut img = ImageBuffer::new(self.window.width,
                                   self.window.height);

        for (x, y, pixel) in img.enumerate_pixels_mut() 
        {
            // NOTE(stanisz): Logging info informing about progress
            // of raytracing horizontal lines of an image.
            // 'debug_display_scanlines_multiple' variable 
            // is storing information about the frequency of prints.
            if x == 0 &&
                y % debug_display_scanlines_multiple == 0 
            {
                if y == self.window.height - debug_display_scanlines_multiple
                {
                    println!("Scanning lines: {} - {}", 
                             y, self.window.height)
                }
                else
                {
                    println!("Scanning lines: {} - {}", 
                             y, y + debug_display_scanlines_multiple - 1)
                }
            }
            // --- End debug info.

            let u = x as f32 / (self.window.width -1) as f32;
            let v = y as f32 / (self.window.height -1) as f32;

            let new_x = lerp(self.camera.get_lower_left_canvas().x(),
                            self.camera.get_upper_right_canvas().x(),
                            u);
            let new_y = lerp(self.camera.get_lower_left_canvas().y(),
                            self.camera.get_upper_right_canvas().y(),
                            v);
                                    
            let new_z = self.camera.get_lower_left_canvas().z();

            let current_pixel = Vec3::new(new_x, new_y, new_z);

            let current_ray = Ray::new(self.camera.position,
                                   (current_pixel - self.camera.position).
                                   normalize());

            let mut pixel_color = Vec3::zero();

            for _cur_ray_per_pixel in 1..self.rays_per_pixel + 1
            {
                pixel_color += self.ray_cast(&current_ray) / 
                    (self.rays_per_pixel as f32);
    
            }

            if true
            {
                if pixel_color.x() > 1.0 {pixel_color.set_x(1.0);}
                if pixel_color.y() > 1.0 {pixel_color.set_y(1.0);}
                if pixel_color.z() > 1.0 {pixel_color.set_z(1.0);}
            
                pixel_color.set_x(pixel_color.x().sqrt());
                pixel_color.set_y(pixel_color.y().sqrt());
                pixel_color.set_z(pixel_color.z().sqrt());
            }
            let (color_x, color_y, color_z) = (
                (pixel_color.x() * 255.0 + 0.5) as u8,
                (pixel_color.y() * 255.0 + 0.5) as u8,
                (pixel_color.z() * 255.0 + 0.5) as u8 );

            *pixel = image::Rgb([color_x, color_y, color_z]);

            //*pixel = image::Rgb([lerp(0 as f32, 255 as f32, u) as u8,
            //lerp(0 as f32, 255 as f32, v) as u8, 0]);
        }
        img.save("RustyBeauty.png").unwrap();
    }
}


fn lerp<T>(a: T, b: T, t: f32) -> T
where T:
    std::ops::Mul<f32, Output=T> + 
    std::ops::Add<T, Output=T>
{
    a * (1.0 - t) + b * t
}

fn hadamard(a: &Vec3, b: &Vec3) -> Vec3
{
    let result = Vec3::new(
        a.x() * b.x(),
        a.y() * b.y(),
        a.z() * b.z()
        );

    return result;
}

fn random0_1() -> f32
{
    let mut rng = rand::thread_rng();
    let val: f32 = rng.gen();

    return val;
}

fn random_range(a: f32, b: f32) -> f32
{
    return random0_1() * (b - a) + a;
}

//TODO(staneesh): read about this
fn random_in_unit_sphere() -> Vec3
{
    let a = random_range(0.0, 2.0 * 3.1415);
    let z = random_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}
