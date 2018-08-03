extern crate image;

use std::io;

mod flip;
mod mirror;
mod grayscale;
mod rotate90;
mod rotate180;
mod rotate270;
mod imagestruct;


//Main Function for the project.
fn main() {

    //When the program is called, it give a description of what it does and also ask for the path
    //of the image they wish to change. The program will then attempt to open that image and give
    //options to the user if it is opened correctly or send out an error if it failed
    
    println!("This is an image editor program. It will allow you to alter an image of your choice ");
    println!("using the features available. Please input the path to your image file.");
    let mut image_path = String::new();
    io::stdin().read_line(&mut image_path).expect("String entered in incorrectly");
    println!("{}", image_path);

    //Pop the last char from the input because it is '\n' and that is not needed in the path
    image_path.pop();

    //Create new struct instance of MainImage which will contain the image and its dimensions
    let mut img = imagestruct::MainImage::new(image_path);

    //create path variable
    //let path = "./test_pictures/pexels-photo-248797.jpeg";
    //let path = "./test_pictures/Test.jpg";
    //let path = "Test.jpg";

    //Call the flip function and output function

    //img = img.flip_image();

    //img = img.mirror_image();

    //img = img.grayscale_image();

//	img = img.rotate90_image();
	img = img.rotate180_image();
//	img = img.rotate270_image();

    //Output will place a file in the src directory
    img.output();

}

