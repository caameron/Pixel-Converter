//This file is used to rotate the image by 270 degrees. The logic is similar to rotate
//by 90 degrees except it will be inverted.
extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

pub fn rotate270_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut rotated = ImageBuffer::new(height, width);  //newly rotated image will have opposite dim.

    let mut rot_width = 0;
    let mut rot_height = width - 1;

    //iterate through each pixel of original image and put pixel into newly rotated image.
    for pixel in img_struct.img.pixels() {
        rotated.put_pixel(rot_width, rot_height, pixel.2);
        if rot_height == 0 {
            rot_height = width;
            rot_width += 1;
        }
        rot_height -= 1;
    }
    rotated
}
