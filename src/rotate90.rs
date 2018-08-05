extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

pub fn rotate90_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut rotated = ImageBuffer::new(height, width);

    let mut rot_width = height - 1;
    let mut rot_height = 0;

    for pixel in img_struct.img.pixels() {
        rotated.put_pixel(rot_width, rot_height, pixel.2);
        rot_height += 1;
        if rot_height == width {
            if rot_width != 0 {
                rot_width -= 1;
            }
            rot_height = 0;
        }
    }
    rotated
}
