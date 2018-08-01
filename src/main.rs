extern crate image;

mod flip;

use image::GenericImage;
use std::fs::File;

//Main Function for the project.
fn main() {
    
    //Open image file
    //let img = image::open("Test.jpg").expect("Image cannot be opened"); 
    //println!("dimensions {:?}", img.dimensions());


    let new_img = flip::flip_img("pexels-photo-248797.jpeg".to_string());

    //Save new image as a output.png
    let ref mut fout = File::create("output.png").expect("Cannot create file");
    image::ImageRgba8(new_img).save("output.png").expect("unable to save file"); 
}

