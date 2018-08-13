extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;
use std::io;

//Pixelate image
//Grabs every nthxnth pixels and forms them together to pixelate the image.
pub fn pixelate(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    //Ask user for the size of the new pixels after pixelation
    let mut size = String::new();
    println!("Please pick a pixelation value from 1 to 5. The higher the number the more pixelated the image will become");
    io::stdin()
        .read_line(&mut size)
        .expect("Value entered in incorrectly");
    size.pop();

    let mut big_pix = size.parse::<u32>().unwrap();
    big_pix = big_pix * 2;
    let mut pixelated_img = ImageBuffer::new(width, height);

    //Copy over image to be pixelated
    for wid in 0..width {
        for hei in 0..height {
            let pix = img_struct.img.get_pixel(wid, hei);
            pixelated_img.put_pixel(wid, hei, pix);
        }
    }

    //Ask user if they want to only pixelate a certain area or the whole image
    //Pixelate only a certain area of the img. Similiar to the pixelate function except it will ask the
    //user to input the dimensions of where they want to pixelate it.
    //(Will ask for a starting x y position and how long in the x and y direction they want to go
    //0,0      500 500. this will start at position 0 and go 500 pixels in the x and y directipons

    //Variables to determine pixelation area
    let x_start;
    let y_start;
    let mut x_end;
    let mut y_end;

    let mut choice = String::new();
    println!("Enter 1 if you want to pixelate the whole image or 2 if you want to only pixelate a specific area");
    io::stdin()
        .read_line(&mut choice)
        .expect("Value entered in incorrectly");
    choice.pop();
    let response = choice.parse::<u32>().unwrap();
    if response == 2 {
        choice.clear();
        println!("Enter in the x coordinate of where you want to start pixelation");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        x_start = choice.parse::<u32>().unwrap();
        choice.clear();

        println!("Enter in the y coordinate of where you want to start pixelation");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        y_start = choice.parse::<u32>().unwrap();
        choice.clear();

        println!("Enter in the the width for pixelation");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        x_end = x_start + choice.parse::<u32>().unwrap();
        if x_end > width {
            x_end = width;
        }
        choice.clear();

        println!("Enter in the the height (going downwards) for pixelation");
        io::stdin()
            .read_line(&mut choice)
            .expect("Value entered in incorrectly");
        choice.pop();
        y_end = y_start + choice.parse::<u32>().unwrap();
        if y_end > height {
            y_end = height;
        }
    } else {
        x_start = 0;
        y_start = 0;
        x_end = width;
        y_end = height;
    }

    //Loop through the pixels in the image, grabbing every pixel that will represent a block.
    //That pixel will then be copied over to the other pixels in the block.
    for wid in x_start..x_end {
        for hei in y_start..y_end {
            if (wid % big_pix == 0) && (hei % big_pix == 0) {
                let base_pix = img_struct.img.get_pixel(wid, hei);

                for neighbor_width in wid..wid + big_pix + 1 {
                    for neighbor_height in hei..hei + big_pix + 1 {
                        if (neighbor_width < x_end) && (neighbor_height < y_end) {
                            pixelated_img.put_pixel(neighbor_width, neighbor_height, base_pix);
                        }
                    }
                }
            }
        }
    }
    pixelated_img
}

//Seperate function which is exactly the same as the pixelate function above however this version
//does not have input output and will strictly be used for testing purposes
#[allow(dead_code)]
pub fn pixelate_test(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let big_pix = 6;
    let mut pixelated_img = ImageBuffer::new(width, height);

    //Copy over image to be pixelated
    for wid in 0..width {
        for hei in 0..height {
            let pix = img_struct.img.get_pixel(wid, hei);
            pixelated_img.put_pixel(wid, hei, pix);
        }
    }

    //Variables to determine pixelation area
    let x_start;
    let y_start;
    let x_end;
    let y_end;
    
    x_start = 0;
    y_start = 0;
    x_end = width;
    y_end = height;

    //Loop through the pixels in the image, grabbing every pixel that will represent a block.
    //That pixel will then be copied over to the other pixels in the block.
    for wid in x_start..x_end {
        for hei in y_start..y_end {
            if (wid % big_pix == 0) && (hei % big_pix == 0) {
                let base_pix = img_struct.img.get_pixel(wid, hei);

                for neighbor_width in wid..wid + big_pix + 1 {
                    for neighbor_height in hei..hei + big_pix + 1 {
                        if (neighbor_width < x_end) && (neighbor_height < y_end) {
                            pixelated_img.put_pixel(neighbor_width, neighbor_height, base_pix);
                        }
                    }
                }
            }
        }
    }
    pixelated_img
}

//Seperate function that will be used for testing purposes. Specifically it will be used to test
//pixelating a certain area
#[allow(dead_code)]
pub fn pixelate_area_test(img_struct: &imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = img_struct.width;
    let height = img_struct.height;

    let big_pix = 6;
    let mut pixelated_img = ImageBuffer::new(width, height);

    //Copy over image to be pixelated
    for wid in 0..width {
        for hei in 0..height {
            let pix = img_struct.img.get_pixel(wid, hei);
            pixelated_img.put_pixel(wid, hei, pix);
        }
    }

    //Variables to determine pixelation area
    let x_start;
    let y_start;
    let x_end;
    let y_end;

    x_start = 100;
    y_start = 100;
    x_end = 200;
    y_end = 200;

    //Loop through the pixels in the image, grabbing every pixel that will represent a block.
    //That pixel will then be copied over to the other pixels in the block.
    for wid in x_start..x_end {
        for hei in y_start..y_end {
            if (wid % big_pix == 0) && (hei % big_pix == 0) {
                let base_pix = img_struct.img.get_pixel(wid, hei);

                for neighbor_width in wid..wid + big_pix + 1 {
                    for neighbor_height in hei..hei + big_pix + 1 {
                        if (neighbor_width < x_end) && (neighbor_height < y_end) {
                            pixelated_img.put_pixel(neighbor_width, neighbor_height, base_pix);
                        }
                    }
                }
            }
        }
    }
    pixelated_img
}
