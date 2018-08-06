//File will contain the code for the struct that will hold the image passed in from the user. The
//struct will contain functions that will alter the image.

use std::io;

extern crate image;
extern crate rand;

use enlarge;
use flip;
use grayscale;
use image::{DynamicImage, GenericImage};
use jumbler;
use mirror;
use pixelate;
use rotate180;
use rotate270;
use rotate90;

//Struct to hold information about image so that it can be passed around from function to function
pub struct MainImage {
    pub img: DynamicImage,
    pub width: u32,
    pub height: u32,
}

//Implement functions for MainImage struct
impl MainImage {
    //Create a new MainImage struct by passing in the images path. It will then be opened and its
    //dimensions will be stored for use in functions. The constructor first opens the image and
    //places it in a temporary variable. The width and height are also placed in temp variables. We
    //then use the shortcut contstructor for a struct and return it
    pub fn new(image_path: String) -> MainImage {
        let new_image = image::open(&image_path).expect("Image cannot be opened");
        let (img_width, img_height) = new_image.dimensions();
        let return_image = MainImage {
            img: new_image,
            width: img_width,
            height: img_height,
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
   //also saves new dimensions.
   pub fn rotate90_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate90::rotate90_img(&self));
	let (width,height) = self.img.dimensions();
	self.width = width;
	self.height = height;
	self
   }

    //Function that will rotate picture by 180 degrees clockwise. 
   pub fn rotate180_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate180::rotate180_img(&self));
	self
   }
 
    //Function that will rotate picture by 270 degrees clockwise. 
    //also saves new dimensions.
   pub fn rotate270_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(rotate270::rotate270_img(&self));
	let (width,height) = self.img.dimensions();
	self.width = width;
	self.height = height;
	self
   }

   //Function that will choose which rotation to execute
   pub fn choose_rotation(self) -> MainImage {
       //Ask for what degree the user wants to rotate their image
       let mut degree = String::new();
       println!("Choose what degree you want to rotate your picture. 90, 180, 270?");
       io::stdin().read_line(&mut degree).expect("Choice not entered in correctly");
       degree.pop();

       //Match user choice with a function to execute
       
       let return_self = match degree.as_ref() {
           "90"  => self.rotate90_image(),
           "180" => self.rotate180_image(),
           "270" => self.rotate270_image(),
           _     => {
               println!("NOT A VALID CHOICE");
               self
            },
       };
       return_self
   }

   //Function that will randomize pixels of image to create jumbled image
   pub fn jumbler_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(jumbler::jumbler_img(&self));
	self
   }

   //Function that will double the size of the image.
   //also saves new dimensions.
   pub fn enlarge_image(mut self) -> MainImage {
	self.img = image::ImageRgba8(enlarge::enlarge_img(&self));
	let (width,height) = self.img.dimensions();
	self.width = width;
	self.height = height;
	self
   }
    //Function that will pixelate the whole image
    pub fn pixelate_img(mut self) -> MainImage {
        self.img = image::ImageRgba8(pixelate::pixelate(&self));
        self
    }
}
