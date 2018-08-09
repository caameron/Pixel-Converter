//this file contains the function to grayscale an image.
extern crate image;
use image::{GenericImage, ImageBuffer, Pixel, Rgba};
use imagestruct;

pub fn grayscale_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut grayed = ImageBuffer::new(width, height);

    let mut g_width = 0;
    let mut g_height = 0;

    //iterate through the pixels and place the pixels in same coordinant location
    //except convert pixel to grayscale.
    for pixel in img_struct.img.pixels() {
        grayed.put_pixel(g_width, g_height, pixel.2.to_luma().to_rgba());
        g_width += 1;
        if g_width >= width {
            g_width = 0;
            g_height += 1;
        }
    }
    grayed
}
