use glam::{Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Material
{
    pub color: Vec3,
    pub is_a_source: bool,
    pub shininess: f32,
}

impl Material
{
    pub fn new(color: (f32, f32, f32),
                is_a_source: bool,
               shininess: f32) -> Material
    {
        Material
        {
            color: Vec3::new(color.0,
                                     color.1,
                                     color.2),

            is_a_source,
            shininess,
        }
    }
}

