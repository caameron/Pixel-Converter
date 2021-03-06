//This file is used for the rotate image by 180 degrees.
extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

pub fn rotate180_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut rotated = ImageBuffer::new(width, height); //new image will have same dimensions as original

    let mut rot_width = width - 1;
    let mut rot_height = height - 1;

    //iterate through each pixel in original image and put pixel in bottom up fashion for 180 rotate.
    for pixel in img_struct.img.pixels() {
        rotated.put_pixel(rot_width, rot_height, pixel.2);
        if rot_width == 0 {
            rot_width = width;
            if rot_height != 0 {
                rot_height -= 1;
            }
        }
        rot_width -= 1;
    }
    rotated
}
