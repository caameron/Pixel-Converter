//This file will contain the code to flip the image upside down
extern crate image;
use image::{DynamicImage, GenericImage, Pixel, ImageBuffer, Rgba};

//The function will pass in a String that contains the path to the image which will be altered. It
//will then open the function grab its dimensions and go through the each pixel replacing them with
//its opposite.
pub fn flip_img(image_path:String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//pub fn flip_img(image_path:String) -> ImageBuffer<(u32, u32, image::Rgba<u8>), Vec<u8>> {

    //Open image (check for errors upon opening)
    let img = image::open(&image_path).expect("Image cannot be opened");

    println!("dimensions {:?}", img.dimensions());
    //Grab the dimensions of the image to create a new image with
    let (width, height) = img.dimensions();

    //Create a new image that will contain the flipped image
    let mut flipped = ImageBuffer::new(width, height);
    //Iterate through the pixels of the image and copy them over to their opposite of the flipped
    //image.
   
    
    let mut flip_width = 0;
    let mut flip_height = height-1;

    let mut pixel_it = img.pixels().peekable();
    for pixel in img.pixels(){
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
