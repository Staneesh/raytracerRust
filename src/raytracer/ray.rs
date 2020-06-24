use glam::{Vec3};
use super::geometry::sphere::Sphere;

#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    pub position: Vec3,
    pub direction: Vec3,
}

impl Ray
{
    pub fn new(position: Vec3, direction: Vec3) -> Ray
    {
        Ray
        {
            position,
            direction,
        }
    }

    pub fn along(&self, t: f32) -> Vec3
    {
        self.position + self.direction * t
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> Option<Vec3>
    {
        let ray_to_sphere_center = self.position - sphere.position;
        let a = Vec3::dot(self.direction, self.direction);
        let b = 2.0 * Vec3::dot(ray_to_sphere_center, self.direction);
        let c = Vec3::dot(ray_to_sphere_center, ray_to_sphere_center) -
            sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0
        {
            let discriminant_sqrt = discriminant.sqrt();
            let t1 = max_f32(
                (-b - discriminant_sqrt) / (2.0 * a), 0.0
                );
            let t2 = max_f32(
                (-b + discriminant_sqrt) / (2.0 * a), 0.0
                );

            let t = min_f32(t1, t2);

            return Some(self.along(t))
        }
        
        return None;
    }
}

fn max_f32(a: f32, b: f32) -> f32
{
    if a > b
    {
        return a;
    }

    return b;   
}

fn min_f32(a: f32, b: f32) -> f32
{
    return -max_f32(-a, -b);
}

