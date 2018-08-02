extern crate image;
use image::{Rgba, ImageBuffer, GenericImage};
use imagestruct;

pub fn rotate270_img(img_struct:&imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>
{
	let width = img_struct.width;
	let height = img_struct.height;

	let mut rotated = ImageBuffer::new(height,width);

	let mut rot_width = 0;
	let mut rot_height = width-1;

	for pixel in img_struct.img.pixels()
	{
		rotated.put_pixel(rot_width, rot_height, pixel.2);
		if rot_height == 0
		{
			rot_height = width;
			rot_width += 1;
		}
		rot_height -= 1;
	}
	rotated












}
