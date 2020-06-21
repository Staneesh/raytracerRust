use glam::{Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Material
{
    pub diffuse_color: Vec3,
    pub shininess: f32,
}

impl Material
{
    pub fn new(diffuse_r: f32, diffuse_g: f32, diffuse_b: f32, 
               shininess: f32) -> Material
    {
        Material
        {
            diffuse_color: Vec3::new(diffuse_r, diffuse_g, diffuse_b),
            shininess,
        }
    }
}

