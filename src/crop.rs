extern crate image;
use image::{GenericImage, ImageBuffer, Rgba};
use imagestruct;
use std::io;

//Pixelate image
//Grabs every nthxnth pixels and forms them together to pixelate the image.
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

    while flag == 0
    {
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

	    if x_start > x_end || y_start > y_end
	    {
		flag = 1;
	    }

	    if (x_end + 1) > width || (y_end + 1) > height
	    {
		println!("{}, {}", x_end, y_end);
		flag = 0;
	    }
	    if flag == 0
	    {
		println!("Invalid coordinants to crop, try again");
            }

    }

    let cropwidth = x_end - x_start + 1;
    let cropheight = y_end - y_start + 1;

    let mut cropped_img = ImageBuffer::new(cropwidth, cropheight);


    let mut crop_x = 0;
    let mut crop_y = 0;
    let mut rel_x = x_start;
    let mut rel_y = y_start;
    for pixel in img_struct.img.pixels()
    {
	if pixel.0 == rel_x && pixel.1 == rel_y
	{
		cropped_img.put_pixel(crop_x, crop_y, pixel.2);
		crop_x += 1;
		if crop_x == cropwidth as u32
		{
			crop_x = 0;
			crop_y += 1;
		}
		rel_x += 1;
		if rel_x > x_end
		{
			rel_x = x_start;
			rel_y += 1;
		}
	}
	if crop_y == cropheight
	{
		break;
	}
    }
    cropped_img

}








