extern crate image;
use image::{Rgba, ImageBuffer, GenericImage};
use imagestruct;

pub fn mirror_img(img_struct:&imagestruct::MainImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>
{
	let width = img_struct.width;
	let height = img_struct.height;

	let mut mirrored = ImageBuffer::new(width,height);

	let mut mir_width = width-1;
	let mut mir_height = 0;

	for pixel in img_struct.img.pixels()
	{
		mirrored.put_pixel(mir_width, mir_height, pixel.2);
		if mir_width == 0
		{
			mir_width = width;
			mir_height += 1;
		}
		mir_width -= 1;
	}
	mirrored












}
