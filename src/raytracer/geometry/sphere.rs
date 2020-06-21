use glam::{Vec2, Vec3};

#[derive(Debug)]
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

