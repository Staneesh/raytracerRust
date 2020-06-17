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
    fn new(position: Vec3, direction: Vec3) -> Ray
    {
        Ray
        {
            position,
            direction,
        }
    }

    fn along(&self, t: f32) -> Vec3
    {
        self.position + self.direction * t
    }
}

impl Camera
{
    fn new(position: Vec3, direction: Vec3,
            canvas_dimensions: Vec2, canvas_distance: f32) -> Camera
    {
        Camera
        {
            position, direction,
            canvas_dimensions, canvas_distance, 
        }
    }

    fn canvas_origin(&self) -> Vec3
    {
        self.position + self.direction * self.canvas_distance
    }
}

fn lerp<T>(a: T, b: T, t: f32) -> T
where T:
    std::ops::Mul<f32, Output=T> + 
    std::ops::Add<T, Output=T>
{
    a * (1.0 - t) + b * t
}

fn ray_cast(ray: Ray) -> (u8, u8, u8)
{
    (255, 123, 14)
}

fn main()
{
    let width = 512;
    let height = 512;
    let aspect_ratio = width as f32 / height as f32;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0),
        Vec2::new(aspect_ratio, 1.0), 1.0
        );   


    // TODO(stanisz): this only works if camera.direction
    // == (0, 0, -1)!
    let lower_left_canvas = camera.canvas_origin() - 
        Vec3::new(camera.canvas_dimensions.x() as f32 / 2 as f32,
                  camera.canvas_dimensions.y() as f32 / 2 as f32,
                  0.0);

    let upper_right_canvas = lower_left_canvas + 
        Vec3::new(camera.canvas_dimensions.x(), 
                  camera.canvas_dimensions.y(),
                  0.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() 
    {
        let u = x as f32 / (width-1) as f32;
        let v = y as f32 / (height-1) as f32;

        let new_x = lerp(lower_left_canvas.x(),
                        upper_right_canvas.x(),
                        u);
        let new_y = lerp(lower_left_canvas.y(),
                        upper_right_canvas.y(),
                        v);
                                
        let new_z = lower_left_canvas.z();

        let current_pixel = Vec3::new(new_x, new_y, new_z);

        let current_ray = Ray::new(camera.position,
                               current_pixel - camera.position);

        let (color_x, color_y, color_z) = ray_cast(current_ray);
        *pixel = image::Rgb([color_x, color_y, color_z]);

        //*pixel = image::Rgb([lerp(0 as f32, 255 as f32, u) as u8,
        //lerp(0 as f32, 255 as f32, v) as u8, 0]);
    }
    
    img.save("RustyBeauty.png").unwrap();

}
