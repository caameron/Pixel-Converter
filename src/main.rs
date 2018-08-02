extern crate image;

mod flip;
mod imagestruct;

//Main Function for the project.
fn main() {

    //create path variable
    let path = "./test_pictures/pexels-photo-248797.jpeg";
    //let path = "./test_pictures/Test.jpg";
    //Make a new struct instance of MainImage
    let mut img = imagestruct::MainImage::new(path.to_string());

    //Call the flip function and output function
    img = img.flip_image();
    //Output will place a file in the src directory
    img.output();

}

