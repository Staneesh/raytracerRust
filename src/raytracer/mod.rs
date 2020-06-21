pub mod geometry;
pub mod material;
mod camera;
mod ray;
mod window;

use window::Window;
use camera::Camera;
use glam::{Vec2, Vec3};

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
                )
            
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
}

