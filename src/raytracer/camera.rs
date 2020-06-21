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

    pub fn canvas_origin(&self) -> Vec3
    {
        self.position + self.direction * self.canvas_distance
    }
}
