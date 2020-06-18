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

#[derive(Debug)]
struct Sphere
{
    position: Vec3,
    radius: f32
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

    fn hit_sphere(&self, sphere: &Sphere) -> bool
    {
        let ray_to_sphere_center = self.position - sphere.position;
        let a = Vec3::dot(self.direction, self.direction);
        let b = 2.0 * Vec3::dot(ray_to_sphere_center, self.direction);
        let c = Vec3::dot(ray_to_sphere_center, ray_to_sphere_center) -
            sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
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

impl Sphere
{
    fn new(position: Vec3, radius: f32) -> Sphere
    {
        Sphere
        {
            position, radius
        }
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
    let test_sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 2.0);
    if ray.hit_sphere(&test_sphere)
    {
        return (255, 123, 14)
    }

    return (0, 0, 0)
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

    let debug_display_scanlines_multiple = 16;

    for (x, y, pixel) in img.enumerate_pixels_mut() 
    {
        if x == 0 &&
            y % debug_display_scanlines_multiple == 0 
        {
            if y == height - debug_display_scanlines_multiple
            {
                println!("Scanning lines: {} - {}", 
                         y, height)
            }
            else
            {
                println!("Scanning lines: {} - {}", 
                         y, y + debug_display_scanlines_multiple - 1)
            }
        }
        
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
