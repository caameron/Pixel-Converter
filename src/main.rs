extern crate image;
extern crate rand;

use std::io;
mod crop;
mod enlarge;
mod flip;
mod grayscale;
mod imagestruct;
mod jumbler;
mod mirror;
mod pixelate;
mod rotate180;
mod rotate270;
mod rotate90;

//Main Function for the project.
fn main() {
    //When the program is called, it give a description of what it does and also ask for the path
    //of the image they wish to change. The program will then attempt to open that image and give
    //options to the user if it is opened correctly or send out an error if it failed

    println!(
        "This is an image editor program. It will allow you to alter an image of your choice "
    );
    println!("using the features available. Please input the path to your image file.");
    let mut image_path = String::new();
    io::stdin()
        .read_line(&mut image_path)
        .expect("String entered in incorrectly");

    //Pop the last char from the input because it is '\n' and that is not needed in the path
    image_path.pop();

   
    //Create new struct instance of MainImage which will contain the image and its dimensions
    let mut img = imagestruct::MainImage::new(image_path);

    //Print out the menu for the user to choose from. Each feature will have a number associated
    //with it and the user will simply just have to input the number of the feature they want to
    //use. The user will be allowed to perform multiple alterations so the program will only end
    //when the user enters in 0
    //DEVELOPMENT: Any addition features can just be added on to the match statement and a new
    //println! with have to be added in as well

    //Bool variable that will let the menu code know whether or not to keep printing
    let mut continue_menu = true;
    let mut choice = String::new();

    //While loop that will keep printing the menu until user is done
    while continue_menu == true {
        println!("What would you like to be done to your image?");
        println!("1 : Flip\n2 : Mirror\n3 : Grayscale\n4 : Rotate\n5 : Jumble\n6 : Enlarge");
        println!("7 : Pixelate");
        println!("8 : Crop");
        println!("0 : EXIT PROGRAM");
        //Can place new menu items here

        println!("Enter in the number of the feature: ");
        io::stdin()
            .read_line(&mut choice)
            .expect("Choice not entered in correctly");
        choice.pop();

        //match statement to see which choice the user entered and to call the correct function
        //based on their answer
        match choice.as_ref() {
            "0" => continue_menu = false,
            "1" => img = img.flip_image(),
            "2" => img = img.mirror_image(),
            "3" => img = img.grayscale_image(),
            "4" => img = img.choose_rotation(),
            "5" => img = img.jumbler_image(),
            "6" => img = img.enlarge_image(),
            "7" => img = img.pixelate_img(),
            "8" => img = img.crop_img(),
            _ => println!("Not a valid choice, please choose again."),
        }
        choice.clear();
    }

    println!("PROGRAM EXITING...");
    //Output will place a file in the src director
    img.output();
}


//Unit tests that test each individual feature.
#[cfg(test)]
mod flip_test {
    extern crate image;
    use image::GenericImage;
    use imagestruct;
    use pixelate;
    use rotate90;
    use rotate180;
    use rotate270;
    use crop;
  
    #[test]
    fn test_flip() {
        let flip_image = image::open("./src/test_pictures/flip.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img = test_img.flip_image();
  
        assert_eq!(flip_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_gray() {
        let gray_image = image::open("./src/test_pictures/gray.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img = test_img.grayscale_image();
  
        assert_eq!(gray_image.pixels().eq(test_img.img.pixels()), true);
    }     
    
    #[test]
    fn test_enlarge() {
        let enlarge_image = image::open("./src/test_pictures/enlarge.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img = test_img.enlarge_image();
  
        assert_eq!(enlarge_image.pixels().eq(test_img.img.pixels()), true);
    }
    
    #[test]
    fn test_mirror() {
        let mirror_image = image::open("./src/test_pictures/mirror.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img = test_img.mirror_image();
  
        assert_eq!(mirror_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_pixelate() {
        let pix_image = image::open("./src/test_pictures/pixel_area.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img.img = image::ImageRgba8(pixelate::pixelate_area_test(&test_img));
  
        assert_eq!(pix_image.pixels().eq(test_img.img.pixels()), true);
       
        let test_image = image::open("./src/test_pictures/pixel_whole.png".to_string()).expect("TEST ERROR");
        test_img.img = image::ImageRgba8(pixelate::pixelate_test(&test_img));
        assert_eq!(test_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_crop() {
	let crop_image = image::open("./src/test_pictures/crop_test.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img.img = image::ImageRgba8(crop::crop_test(&test_img));
	assert_eq!(crop_image.pixels().eq(test_img.img.pixels()), true);  
    }
   
    #[test]
    fn test_rotate() {
        let r90_image = image::open("./src/test_pictures/rotate_90.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img.img = image::ImageRgba8(rotate90::rotate90_img(&test_img));
  
        assert_eq!(r90_image.pixels().eq(test_img.img.pixels()), true);
       
        let r180_image = image::open("./src/test_pictures/rotate_180.png".to_string()).expect("TEST ERROR");
        test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img.img = image::ImageRgba8(rotate180::rotate180_img(&test_img));
        assert_eq!(r180_image.pixels().eq(test_img.img.pixels()), true); 
        
        let r270_image = image::open("./src/test_pictures/rotate_270.png".to_string()).expect("TEST ERROR");
        test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img.img = image::ImageRgba8(rotate270::rotate270_img(&test_img));
        assert_eq!(r270_image.pixels().eq(test_img.img.pixels()), true);

    }

    #[test]
    fn should_fail() {
        let fail_image = image::open("./src/test_pictures/rotate_180.png".to_string()).expect("TEST ERROR");
        let mut test_img = imagestruct::MainImage::new("./src/test_pictures/Test.jpg".to_string());
        test_img.img = image::ImageRgba8(rotate90::rotate90_img(&test_img));
        assert_eq!(fail_image.pixels().eq(test_img.img.pixels()), false);
    }

    #[test]
    fn test_flip_mirror_gray_enlarge() {
	let all_image = image::open("./src/test_pictures/flip_mirror_gray_enlarge.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.flip_image();
	test_img = test_img.mirror_image();
	test_img = test_img.grayscale_image();
	test_img = test_img.enlarge_image();
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_flip_enlarge() {
	let all_image = image::open("./src/test_pictures/flip_enlarge.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.flip_image();
	test_img = test_img.enlarge_image();
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_flip_gray() {
	let all_image = image::open("./src/test_pictures/flip_gray.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.flip_image();
	test_img = test_img.grayscale_image();
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_flip_mirror() {
	let all_image = image::open("./src/test_pictures/flip_mirror.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.flip_image();
	test_img = test_img.mirror_image();
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_mirror_gray_crop() {
	let all_image = image::open("./src/test_pictures/mirror_gray_crop.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.mirror_image();
	test_img = test_img.grayscale_image();
	test_img.img = image::ImageRgba8(crop::crop_test(&test_img));
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_gray_mirror_pixelate() {
	let all_image = image::open("./src/test_pictures/gray_mirror_pixel.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.grayscale_image();
	test_img = test_img.mirror_image();
	test_img.img = image::ImageRgba8(pixelate::pixelate_test(&test_img));
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }

    #[test]
    fn test_flip_gray_rot90() {
	let all_image = image::open("./src/test_pictures/flip_gray_rot90.png".to_string()).expect("TEST ERROR");
	let mut test_img = imagestruct::MainImage::new("./src/test_pictures/beach.jpeg".to_string());
	test_img = test_img.flip_image();
	test_img = test_img.grayscale_image();
	test_img.img = image::ImageRgba8(rotate90::rotate90_img(&test_img));
	assert_eq!(all_image.pixels().eq(test_img.img.pixels()), true);
    }
    

    
}
