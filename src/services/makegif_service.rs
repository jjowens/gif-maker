use std::any::Any;
use image::{Frame, Frames, ImageBuffer, ImageFormat, ImageResult, open};
use crate::services::helper;

pub fn make_gif(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    println!("- Making GIF");
    println!("- Open Directory: {}", open_file_directory);
    println!("- Save GIF as: {}", save_file_path);

    let vecs : Vec<String> = helper::get_images_from_directory(open_file_directory);

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(100, 100);
    let mut list_of_frames: Vec<Frame> = vec![];
    let mut boxed_frames: Box<ImageResult<Frame>>;
    let mut boxed_frame_alt: Box<dyn Any> = Box::new(5);

    for img in vecs {
        //let currentFrame = image::Frame::new(/* ImageBuffer<Rgba<u8>, Vec<u8>> */)
        let current_image = open(img).unwrap().into_rgba8();
        //let currentFrame = image::Frame::new(ImageBuffer::new(100,100));
        let current_frame = image::Frame::new(current_image);
        list_of_frames.push(current_frame);
    }

    imgbuf.save_with_format(save_file_path, ImageFormat::Gif).unwrap();

    Ok(())
}

pub fn make_gif_old(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    println!("- Making GIF");
    println!("- Open Directory: {}", open_file_directory);
    println!("- Save GIF as: {}", save_file_path);

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(100, 100);

    imgbuf.save_with_format(save_file_path, ImageFormat::Gif).unwrap();

    Ok(())
}
