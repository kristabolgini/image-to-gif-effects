extern crate image;

pub mod zoom {

	use gif::{Encoder, Repeat, SetParameter};
	use gif;

	use image::{GenericImage, DynamicImage, Frames, FilterType};
	use image;
	use std::fs::File;
	use std::borrow::Cow;
	
	pub fn process(source: DynamicImage, output: String) {

		let frame_array: Vec<image::Frame> = Vec::new();
		frame_array.push(image::Frame::new(*source.as_rgba8().unwrap())); // Push first frame

		let width = source.width();
		let height = source.height();

		let frames = 180;
		let seconds = 3;

		let resized: DynamicImage;
		for i in 0..frames {
			
			resized = source.resize(width + 10, height + 10, FilterType::Gaussian);
			let cropped: DynamicImage = resized.crop(0, 0, width, height);
			frame_array.push(image::Frame::new(*cropped.as_rgba8().unwrap()));

		}

		let frames: Frames = Frames::new(frame_array);

		let mut image = File::create(output).unwrap();
		let gif_encoder = Encoder::new(&mut image, source.width() as u16, source.height() as u16, &[]).unwrap();
		gif_encoder.set(Repeat::Infinite).unwrap();


		for frame in frames {
			// let gif_frame = gif::Frame::from_rgba(source.width() as u16, source.height() as u16, frame.into_buffer());
			let gif_frame = gif::Frame::default();
			gif_frame.buffer = Cow::Borrowed(frame.buffer);
			gif_frame.height = source.height() as u16;
			gif_frame.width  = source.width() as u16;
			gif_encoder.write_frame(&gif_frame);
		}


	}

}

pub mod shake {
	extern crate image;
	use image::{GenericImage, DynamicImage, Frame, Frames, FilterType};	
	pub fn process(source: image::DynamicImage, output: String) {}
}