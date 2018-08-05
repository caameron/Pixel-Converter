extern crate image;
use image::{Rgba, ImageBuffer, GenericImage};
use imagestruct;
use std::io;

//Pixelate image
//Grabs every nthxnth pixels and forms them together to pixelate the image.
pub fn pixelate(img_struct:&imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>
{
	let width = img_struct.width;
	let height = img_struct.height;

    //Ask user for the size of the new pixels after pixelation
    let mut size = String::new();
    println!("Please pick a pixelation value from 1 to 5. The higher the number the more pixelated the image will become");
    io::stdin().read_line(&mut size).expect("Value entered in incorrectly");
    size.pop();

    let mut big_pix = size.parse::<u32>().unwrap(); 
    big_pix = big_pix*2;
	let mut pixelated_img = ImageBuffer::new(width,height);


    //Loop through the pixels in the image, grabbing every pixel that will represent a 10x10 block.
    //That pixel will then be copied over to the other pixels in the 10x10 block.
    for wid in 0..width {
        for hei in 0..height {
            if (wid % big_pix == 0) && (hei % big_pix == 0) {
                let base_pix = img_struct.img.get_pixel(wid, hei);
                
                for neighbor_width in wid..wid+big_pix+1 {
                    for neighbor_height in hei..hei+big_pix+1 {
                        if (neighbor_width  < width) && (neighbor_height < height) {
                            pixelated_img.put_pixel(neighbor_width, neighbor_height, base_pix);
                        }
                    }
                }
            }
        }
    }
    pixelated_img

}


//Pixelate only a certain area of the img. Similiar to the pixelate function except it will ask the
//user to input the dimensions of where they want to pixelate it.
//(Will ask for a starting x y position and how long in the x and y direction they want to go
//0,0      500 500. this will start at position 0 and go 500 pixels in the x and y directipons

