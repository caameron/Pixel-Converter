//File will contain the code for the struct that will hold the image passed in from the user. The
//struct will contain functions that will alter the image.

use std::fs::File;

extern crate image;

use image::{DynamicImage, GenericImage};
use flip;
use mirror;
use grayscale;
use rotate90;
use rotate180;
use rotate270;


//Struct to hold information about image so that it can be passed around from function to function
pub struct MainImage {
    pub img:    DynamicImage,  
    pub width:  u32,
    pub height: u32,
}

//Implement functions for MainImage struct
impl MainImage {
    //Create a new MainImage struct by passing in the images path. It will then be opened and its
    //dimensions will be stored for use in functions. The constructor first opens the image and
    //places it in a temporary variable. The width and height are also placed in temp variables. We
    //then use the shortcut contstructor for a struct and return it
    pub fn new(image_path:String) -> MainImage {
        let new_image = image::open(&image_path).expect("Image cannot be opened");
        let (img_width, img_height) = new_image.dimensions();
        let return_image = MainImage {
            img: new_image,
            width:img_width,
            height:img_height,
        };
        return_image 
    }
   
   //Function to output the image file
    pub fn output(self) {
        //Create and Save new image file as a output.png it will be located in the src directory

        //it works without this but i'm keeping it here for reference just in case
        //let ref mut fout = File::create("output.png").expect("Cannot create file");
        self.img.save("output.png").expect("unable to save file"); 
    }
    
    
   //Function that will flip the image on the x axis. It will call a helper function called
   //flip_image
   pub fn flip_image(mut self) -> MainImage {
       self.img = image::ImageRgba8(flip::flip_img(&self));
       self
   }

   //Function that will mirror the image on the y axis. 
   pub fn mirror_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(mirror::mirror_img(&self));
	self
   }

   //Function will gray out the image.
   pub fn grayscale_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(grayscale::grayscale_img(&self));
	self
   }

   //Function that will rotate picture by 90 degrees clockwise. 
   pub fn rotate90_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate90::rotate90_img(&self));
	self
   }

    //Function that will rotate picture by 180 degrees clockwise. 
   pub fn rotate180_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate180::rotate180_img(&self));
	self
   }
 
    //Function that will rotate picture by 270 degrees clockwise. 
   pub fn rotate270_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate270::rotate270_img(&self));
	self
   }

}
