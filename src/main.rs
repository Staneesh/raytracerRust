extern crate image;

use image::{ImageBuffer, RgbImage};
use glam::{Vec3, Vec2};

#[derive(Debug)]
struct Ray
{
    position: Vec3,
    direction: Vec3,
}

#[derive(Debug)]
struct Camera
{
    position: Vec3,
    direction: Vec3,
    canvas_dimensions: Vec2,
    canvas_distance: f32,
}

impl Ray
{
    fn init(position: Vec3, direction: Vec3) -> Ray
    {
        Ray
        {
            position,
            direction,
        }
    }

    fn new() -> Ray
    {
        Ray
        {
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn from(ray: Ray) -> Ray
    {
        Ray
        {
            position: ray.position,
            direction: ray.direction,
        }
    }

    fn along(&self, t: f32) -> Vec3
    {
        self.position + self.direction * t
    }
}

impl Camera
{
    fn init(position: Vec3, direction: Vec3,
            canvas_dimensions: Vec2, canvas_distance: f32) -> Camera
    {
        Camera
        {
            position, direction,
            canvas_dimensions, canvas_distance, 
        }
    }

    fn new() -> Camera
    {
        Camera
        {
            position: Vec3::new(2.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            canvas_dimensions: Vec2::new(1.0, 1.0),
            canvas_distance: 1.0,
        }
    }

    fn from(camera: Camera) -> Camera
    {
        Camera
        {
            position: camera.position,
            direction: camera.direction,
            canvas_dimensions: camera.canvas_dimensions,
            canvas_distance: camera.canvas_distance,
        }
    }

}

fn main()
{
    let width = 512;
    let height = 512;
    let aspect_ratio = width as f32 / height as f32;

    let mut img: RgbImage = ImageBuffer::new(width, height);

      


    for pixel in img.pixels_mut() 
    {
        *pixel = image::Rgb([255, 0, 0]);
    }
    
    img.save("RustyBeauty.png").unwrap();

}
