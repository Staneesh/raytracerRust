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

pub struct Stray
{
    window: Window,
    camera: Camera,

    //NOTE(staneesh): sphere and its material index
    spheres: Vec<(Sphere, u32)>,
    //NOTE(staneesh): material and its index    
    materials: Vec<(Material, u32)>,

    tracing_depth: u32,
}

impl Stray
{
    pub fn new() -> Stray
    {
        Stray
        {
            window: Window::new(512, 512),
            camera: Camera::new(
                Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
                Vec2::new(1.0, 1.0), 1.0
                ),

            //TODO(staneesh): should this be initialized
            // using with_capacity?
            spheres: Vec::<(Sphere, u32)>::new(),
            materials: Vec::<(Material, u32)>::new(),
            tracing_depth: 5,
        }
    }
    pub fn set_window_dimensions(&mut self, width: u32, height: u32)
    {
        self.window = Window::new(width, height);
    }

    pub fn add_sphere(&mut self, x: f32, y: f32, z: f32, r: f32,
                      mat_index: u32)
        -> Result<(), &'static str>
    {
        let sphere = Sphere::new(Vec3::new(x, y, z), r);
        if self.find_material_by_index(&mat_index).is_none()
        {
            return Err("No material of such index!");
        }
        
        self.spheres.push((sphere, mat_index));
        return Ok(());
    }

    pub fn add_material(&mut self,
                        diffuse_r: f32, diffuse_g: f32, diffuse_b: f32,
                        shininess: f32, mat_index: u32)
        -> Result<(), &'static str>
    {
        if self.find_material_by_index(&mat_index).is_some()
        {
            return Err("Cannot add another \
                       material with the same index!");
        }
        
        let material = Material::new(diffuse_r, diffuse_g, diffuse_b,
                                     shininess);
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
    
    fn ray_cast(&self, ray: Ray, cur_depth: u32,
                ray_energy: Vec3) -> Vec3
    {
        if cur_depth > self.tracing_depth
        {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        let mut ray_energy = Vec3::new(1.0, 1.0, 1.0);

        for (_index, (test_sphere, mat_index)) 
            in self.spheres.iter().enumerate()
        {
            if let Some(hit_sphere_point) = ray.hit_sphere(&test_sphere)
            {
                let normal_to_sphere_surface = (hit_sphere_point - 
                    test_sphere.position).normalize();

                let contrib = Vec3::dot(
                    ray.direction.normalize(),
                    normal_to_sphere_surface
                        );

                let material_hit = 
                    self.find_material_by_index(mat_index).unwrap();

                let diffuse_lighting = contrib *  
                    material_hit.diffuse_color;    

                color = color + hadamard(&(255.0 * diffuse_lighting),
                                         &ray_energy);

                ray_energy = hadamard(&ray_energy, &diffuse_lighting);

                let prefect_reflection = ray.direction - 
                    2.0*(contrib) * normal_to_sphere_surface;

                let new_dir = lerp::<Vec3>(
                    random_in_unit_sphere(), prefect_reflection,
                    material_hit.shininess);

                let new_ray = Ray::new(hit_sphere_point, new_dir);

                color = color + self.ray_cast(new_ray, cur_depth + 1,
                                         ray_energy);
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
            let v = y as f32 / (self.window.width -1) as f32;

            let new_x = lerp(self.camera.get_lower_left_canvas().x(),
                            self.camera.get_upper_right_canvas().x(),
                            u);
            let new_y = lerp(self.camera.get_lower_left_canvas().y(),
                            self.camera.get_upper_right_canvas().y(),
                            v);
                                    
            let new_z = self.camera.get_lower_left_canvas().z();

            let current_pixel = Vec3::new(new_x, new_y, new_z);

            let current_ray = Ray::new(self.camera.position,
                                   current_pixel - self.camera.position);

            let pixel_color =
                self.ray_cast(current_ray, 1, Vec3::one());
            let (color_x, color_y, color_z) = (
                pixel_color.x() as u8,
                pixel_color.y() as u8,
                pixel_color.z() as u8 );

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

//TODO(staneesh): implement this
fn random_in_unit_sphere() -> Vec3
{
    return Vec3::one();
}
