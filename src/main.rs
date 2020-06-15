extern crate image;

use image::{ImageBuffer, RgbImage};

fn main()
{
    let width = 512;
    let height = 512;
    let mut img: RgbImage = ImageBuffer::new(width, height);


    // Iterate over all pixels in the image.
    for pixel in img.pixels_mut() 
    {
        *pixel = image::Rgb([255, 0, 0]);
    }
    
    img.save("RustyBeauty.png").unwrap();

}
