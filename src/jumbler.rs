extern crate image;
extern crate rand;
use image::{Rgba, ImageBuffer, GenericImage};
use imagestruct;
use rand::Rng;

pub fn jumbler_img(img_struct:&imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>
{
	let width = img_struct.width;
	let height = img_struct.height;
	let mut randcoord: Vec<(usize,usize)> = vec![];
	let mut index = 0;

	let mut jumbled = ImageBuffer::new(width,height);


	for x in 0..(width as usize)
	{
		for y in 0..(height as usize)
		{
			randcoord.push((x,y));
		}
	}

	rand::thread_rng().shuffle(&mut randcoord);	

	for pixel in img_struct.img.pixels()
	{
		jumbled.put_pixel(randcoord[index].0 as u32,randcoord[index].1 as u32, pixel.2);
		index += 1;
	}
	jumbled












}
