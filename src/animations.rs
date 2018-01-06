extern crate image;

pub mod zoom {

	use gif::{Encoder, Repeat, SetParameter};
	use gif;

	use image::{GenericImage, DynamicImage, FilterType};
	use std::fs::File;
	
	pub fn process(source: DynamicImage, output: String) {

		println!("Began processing source.");
		let mut frame_array: Vec<gif::Frame> = Vec::new();
		let first_frame = gif::Frame::from_rgb(source.width() as u16, source.height() as u16, source.raw_pixels().into_iter().as_slice());
		frame_array.push(first_frame); // Push first frame

		let width = source.width();
		let height = source.height();

		let frames = 180;
		let seconds = 3;

		let mut resized: DynamicImage;
		println!("Started Resize Operation");
		for i in 0..frames {
			println!("Processing frame: {:3}", i);
			resized = source.resize(width + 10, height + 10, FilterType::Gaussian);
			let cropped: DynamicImage = resized.crop(0, 0, width, height);
			frame_array.push(gif::Frame::from_rgb(source.width() as u16, source.height() as u16, cropped.raw_pixels().into_iter().as_slice()));

		}

		let mut image = File::create(output).unwrap();
		let mut gif_encoder = Encoder::new(&mut image, source.width() as u16, source.height() as u16, &[]).unwrap();
		gif_encoder.set(Repeat::Infinite).unwrap();

		println!("Started writing data.");
		for frame in frame_array {
			// // let gif_frame = gif::Frame::from_rgba(source.width() as u16, source.height() as u16, frame.into_buffer());
			// let gif_frame = gif::Frame::default();
			// gif_frame.buffer = Cow::Borrowed(frame.buffer);
			// gif_frame.height = source.height() as u16;
			// gif_frame.width  = source.width() as u16;
			gif_encoder.write_frame(&frame).unwrap();
		}

	}

}

pub mod shake {
	extern crate image;
	use image::{GenericImage, DynamicImage, Frame, Frames, FilterType};	
	pub fn process(source: image::DynamicImage, output: String) {}
}