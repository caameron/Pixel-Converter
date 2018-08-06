//This file contains the function to double the size of an image.
extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

pub fn enlarge_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut enlarged = ImageBuffer::new(width * 2, height * 2); //create new image with double dim.

    let mut e_width = 0;
    let mut e_height = 0;

    //iterate through each pixel and place same pixel in 2x2 pixel block in new image to double size.
    for pixel in img_struct.img.pixels() {
        enlarged.put_pixel(e_width, e_height, pixel.2);
        enlarged.put_pixel(e_width + 1, e_height, pixel.2);
        enlarged.put_pixel(e_width, e_height + 1, pixel.2);
        enlarged.put_pixel(e_width + 1, e_height + 1, pixel.2);
        e_width += 2;
        if e_width >= (width * 2) {
            e_width = 0;
            e_height += 2;
        }
    }
    enlarged
}
