#[derive(Debug)]
pub struct Window 
{
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32
}

impl Window
{
    pub fn new(width: u32, height: u32) -> Window
    {
        Window
        {
            width, height,
            aspect_ratio: width as f32 / height as f32,
        }
    }
}
