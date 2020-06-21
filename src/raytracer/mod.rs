pub mod geometry;
pub mod material;
mod camera;
mod ray;
mod window;

use window::Window;
use camera::Camera;
use ray::Ray;
use geometry::sphere::Sphere;

use glam::{Vec2, Vec3};
use image::{ImageBuffer, RgbImage};

pub struct Stray
{
    window: Window,
    camera: Camera,
}

// TODO(staneesh): i shouldnt have getters at all,
// some of the functions neednt be public, 
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
        }
    }
    pub fn set_window_dimensions(&mut self, width: u32, height: u32)
    {
        self.window = Window::new(width, height);
    }
    pub fn get_window_dimensions(&self) -> (u32, u32)
    {
        (self.window.width, self.window.height)
    }
    pub fn get_window_width(&self) -> u32
    {
        self.window.width
    }
    pub fn get_window_height(&self) -> u32
    {
        self.window.height
    }
    pub fn get_aspect_ratio(&self) -> f32
    {
        self.window.aspect_ratio
    }
    pub fn get_lower_left_canvas(&self) -> Vec3
    {
        // TODO(stanisz): this only works if camera.direction
        // == (0, 0, -1)!
        self.camera.canvas_origin() - 
            Vec3::new(self.camera.canvas_dimensions.x(),
            self.camera.canvas_dimensions.y(),
            0.0)
    }
    pub fn get_upper_right_canvas(&self) -> Vec3
    {
        // TODO(stanisz): this only works if camera.direction
        // == (0, 0, -1)!
        self.get_lower_left_canvas() +
            Vec3::new(self.camera.canvas_dimensions.x(),
            self.camera.canvas_dimensions.y(),
            0.0)
    }
    pub fn get_camera_position(&self) -> Vec3
    {
        self.camera.position
    }
    
    fn ray_cast(&self, ray: Ray) -> (u8, u8, u8)
    {
        let test_sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 2.0);
        if let Some(hit_sphere_point) = ray.hit_sphere(&test_sphere)
        {
            let normal_to_sphere_surface = (hit_sphere_point - 
                test_sphere.position).normalize();

            let red = lerp::<f32>(0.0, 255.0, Vec3::dot(normal_to_sphere_surface, 
                                                  ray.direction));
            return (red as u8, 100, 100);
        }

        return (0, 0, 0)
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

            let new_x = lerp(self.get_lower_left_canvas().x(),
                            self.get_upper_right_canvas().x(),
                            u);
            let new_y = lerp(self.get_lower_left_canvas().y(),
                            self.get_upper_right_canvas().y(),
                            v);
                                    
            let new_z = self.get_lower_left_canvas().z();

            let current_pixel = Vec3::new(new_x, new_y, new_z);

            let current_ray = Ray::new(self.camera.position,
                                   current_pixel - self.camera.position);

            let (color_x, color_y, color_z) = self.ray_cast(current_ray);
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

