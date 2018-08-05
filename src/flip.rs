//This file will contain the code to flip the image upside down

extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;

//The function will pass in a String that contains the path to the image which will be altered. It
//will then open the function grab its dimensions and go through the each pixel replacing them with
//its opposite.
pub fn flip_img(img_struct:&imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    //Grab the dimensions of the image to create a new image with
    let width = img_struct.width;
    let height = img_struct.height;

    //Create a new image that will contain the flipped image
    let mut flipped = ImageBuffer::new(width, height);
    //Iterate through the pixels of the image and copy them over to their opposite of the flipped
    //image.
   
    
    let mut flip_width = 0;
    let mut flip_height = height-1;

    for pixel in img_struct.img.pixels(){
        flipped.put_pixel(flip_width, flip_height, pixel.2);
        flip_width += 1;
        if flip_width >= width {
            flip_width = 0;
            if flip_height > 0 {
                flip_height -= 1;
            }
        }
    }
    
    flipped
}


