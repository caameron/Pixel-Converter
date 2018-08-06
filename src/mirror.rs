//This file will implement the function: mirror the image around the y-axis.
extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

pub fn mirror_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut mirrored = ImageBuffer::new(width, height); //initialize new mirrored image.

    let mut mir_width = width - 1;
    let mut mir_height = 0;

    //iterate through each pixel and place pixel in opposite side of new image.
    //opposite in relation to the y-axis. 
    for pixel in img_struct.img.pixels() {
        mirrored.put_pixel(mir_width, mir_height, pixel.2);
        if mir_width == 0 {
            mir_width = width;
            mir_height += 1;
        }
        mir_width -= 1;
    }
    mirrored
}
