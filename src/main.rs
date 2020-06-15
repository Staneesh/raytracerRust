extern crate image;

fn main() {

    let buffer: &[u8] = unimplemented!(); // Generate the image data

    // Save the buffer as "image.png"
    image::save_buffer("image.png", buffer, 800, 600, image::ColorType::Rgb8).unwrap()
}
