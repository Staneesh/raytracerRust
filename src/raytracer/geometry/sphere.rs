use glam::{Vec3};

//NOTE(staneesh): Copy and Clone for 
// pushing to vec<Sphere> in Stray.
#[derive(Debug, Copy, Clone)]
pub struct Sphere
{
    //TODO(staneesh): shouldnt be pub
    pub position: Vec3,
    pub radius: f32
}


impl Sphere
{
    pub fn new(position: Vec3, radius: f32) -> Sphere
    {
        Sphere
        {
            position, radius
        }
    }
}

