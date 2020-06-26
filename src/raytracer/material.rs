use glam::{Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Material
{
    //TODO(stanisz): when i have more info about
    // how the material / emitter data is laid out,
    // i should separate objects and emitters
    // materials among two structs. Now they are differing
    // only by a few fields and this doesnt have 
    // an impact on memory footprint.
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

