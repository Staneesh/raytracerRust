use glam::{Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Material
{
    pub diffuse_color: Vec3,
    pub emit_color: Vec3,
    pub shininess: f32,
}

impl Material
{
    pub fn new(diffuse: (f32, f32, f32),
                emit: (f32, f32, f32),
               shininess: f32) -> Material
    {
        Material
        {
            diffuse_color: Vec3::new(diffuse.0,
                                     diffuse.1,
                                     diffuse.2),
            emit_color: Vec3::new(emit.0,
                                        emit.1,
                                        emit.2),

            shininess,
        }
    }
}

