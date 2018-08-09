//this file contains the function that will randomly jumble up pixels of an image.
extern crate image;
extern crate rand;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;
use rand::Rng;

pub fn jumbler_img(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;
    let mut randcoord: Vec<(usize, usize)> = vec![];
    let mut index = 0;

    let mut jumbled = ImageBuffer::new(width, height);

    //collect all the possible coordinants of the image.
    for x in 0..(width as usize) {
        for y in 0..(height as usize) {
            randcoord.push((x, y));
        }
    }

    //shuffle the coordinants of the image.
    rand::thread_rng().shuffle(&mut randcoord);

    //iterate through the pixels and put them in random coordinants
    for pixel in img_struct.img.pixels() {
        jumbled.put_pixel(
            randcoord[index].0 as u32,
            randcoord[index].1 as u32,
            pixel.2,
        );
        index += 1;
    }
    jumbled
}
