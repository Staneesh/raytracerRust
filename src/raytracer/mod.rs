pub mod geometry;
pub mod material;
mod camera;
mod ray;
mod window;

use window::Window;

pub struct Stray
{
    window: Window,
}

impl Stray
{
    pub fn new() -> Stray
    {
        Stray
        {
            window: Window::new(512, 512)     
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
}

