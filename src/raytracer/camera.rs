use glam::{Vec2, Vec3};

#[derive(Debug)]
pub struct Camera
{
    pub position: Vec3,
    pub direction: Vec3,
    pub canvas_dimensions: Vec2,
    pub canvas_distance: f32,
}

impl Camera
{
    pub fn new(position: Vec3, direction: Vec3,
            canvas_dimensions: Vec2, canvas_distance: f32) -> Camera
    {
        Camera
        {
            position, direction,
            canvas_dimensions, canvas_distance, 
        }
    }

    pub fn get_canvas_origin(&self) -> Vec3
    {
        self.position + self.direction * self.canvas_distance
    }

    pub fn get_lower_left_canvas(&self) -> Vec3
    {
        // TODO(stanisz): this only works if camera.direction
        // == (0, 0, -1)!
        self.get_canvas_origin() - 
            Vec3::new(self.canvas_dimensions.x() as f32 /2.0,
            self.canvas_dimensions.y() as f32 / 2.0,
            0.0)
    }

    pub fn get_upper_right_canvas(&self) -> Vec3
    {
        // TODO(stanisz): this only works if camera.direction
        // == (0, 0, -1)!
        self.get_lower_left_canvas() +
            Vec3::new(self.canvas_dimensions.x(),
            self.canvas_dimensions.y(),
            0.0)
    }

}
