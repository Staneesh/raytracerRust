extern crate image;

use image::{ImageBuffer, RgbImage};
use glam::{Vec3};

#[derive(Debug)]
struct Ray
{
    position: Vec3,
    direction: Vec3,
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
            position: Vec3::new(1.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
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

fn main()
{
    let width = 512;
    let height = 512;
    let mut img: RgbImage = ImageBuffer::new(width, height);




    for pixel in img.pixels_mut() 
    {
        *pixel = image::Rgb([255, 0, 0]);
    }
    
    img.save("RustyBeauty.png").unwrap();

}
