extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;
use std::io;

//Crops the image
//Creates new image with new cropped dimensions and places the appropriate pixels.
pub fn crop(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let mut x_start = 0;
    let mut y_start = 0;
    let mut x_end = width - 1;
    let mut y_end = height - 1;
    let mut flag = 0;
    let mut choice = String::new();

    println!("Original image is {} by {}", width, height);

    //get input from user for cropped dimensions.
    while flag == 0 {
        flag = 1;
        println!("Enter starting x coordinant for crop");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        x_start = choice.parse::<u32>().unwrap();
        choice.clear();

        println!("Enter starting y coordinant for crop");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        y_start = choice.parse::<u32>().unwrap();
        choice.clear();

        println!("Enter ending x coordinant for crop");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        x_end = choice.parse::<u32>().unwrap();
        choice.clear();

        println!("Enter ending y coordinant for crop");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        y_end = choice.parse::<u32>().unwrap();
        choice.clear();

	//if any of dimensions are invalid then ask again.
        if x_start > x_end || y_start > y_end {
            flag = 0;
        }

        if (x_end + 1) > width || (y_end + 1) > height {
            println!("{}, {}", x_end, y_end);
            flag = 0;
        }
        if flag == 0 {
            println!("Invalid coordinants to crop, try again");
        }
    }

    let cropwidth = x_end - x_start + 1;
    let cropheight = y_end - y_start + 1;

    //create new cropped image with new dimensions.
    let mut cropped_img = ImageBuffer::new(cropwidth, cropheight);

    let mut crop_x = 0;
    let mut crop_y = 0;
    let mut rel_x = x_start;
    let mut rel_y = y_start;
  
    //iterate through each pixel and only add the pixel if it is within the bounds of the
    //cropped dimensions.
    for pixel in img_struct.img.pixels() {
        if pixel.0 == rel_x && pixel.1 == rel_y {
            cropped_img.put_pixel(crop_x, crop_y, pixel.2);
            crop_x += 1;
            if crop_x == cropwidth as u32 {
                crop_x = 0;
                crop_y += 1;
            }
            rel_x += 1;
            if rel_x > x_end {
                rel_x = x_start;
                rel_y += 1;
            }
        }
        if crop_y == cropheight {
            break;
        }
    }
    cropped_img
}

//this function is called to test crop function. This function wont take stdin and instead will
//use default values for testing purposes.
#[allow(dead_code)]
pub fn crop_test(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let x_start = 0;
    let y_start = 0;

    println!("Original image is {} by {}", width, height);

    let x_end = 100;
    let y_end = 100;
 
    let cropwidth = x_end - x_start + 1;
    let cropheight = y_end - y_start + 1;

    //create new cropped image with new dimensions.
    let mut cropped_img = ImageBuffer::new(cropwidth, cropheight);

    let mut crop_x = 0;
    let mut crop_y = 0;
    let mut rel_x = x_start;
    let mut rel_y = y_start;
  
    //iterate through each pixel and only add the pixel if it is within the bounds of the
    //cropped dimensions.
    for pixel in img_struct.img.pixels() {
        if pixel.0 == rel_x && pixel.1 == rel_y {
            cropped_img.put_pixel(crop_x, crop_y, pixel.2);
            crop_x += 1;
            if crop_x == cropwidth as u32 {
                crop_x = 0;
                crop_y += 1;
            }
            rel_x += 1;
            if rel_x > x_end {
                rel_x = x_start;
                rel_y += 1;
            }
        }
        if crop_y == cropheight {
            break;
        }
    }
    cropped_img
}
