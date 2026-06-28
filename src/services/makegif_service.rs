use std::net::Shutdown::Write;
use image::{ImageEncoder, ImageFormat, open};
use image::codecs::gif::GifEncoder;
use crate::services::helper;
use crate::services::helper::split_string_into_colour_map;

pub fn make_gif(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    println!("- Making GIF");
    println!("- Open Directory: {}", open_file_directory);
    println!("- Save GIF as: {}", save_file_path);

    let vecs : Vec<String> = helper::get_images_from_directory(open_file_directory);

    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, _>::new(100, 100);

    let w  = imgbuf.as_mut();
    let gifEncoder = GifEncoder::new(w);

    //imgbuf.save_with_format(save_file_path, ImageFormat::Gif).unwrap();

    Ok(())
}

pub fn clean_and_make_custom_gif(open_file_directory: &str, save_file_path: &str, width: u16, height: u16, colour_map: &str) -> Result<(), String> {
    let new_colour_map = split_string_into_colour_map(colour_map);

    make_custom_gif(open_file_directory, save_file_path, width, height, new_colour_map)?;
    Ok(())
}

pub fn make_custom_gif(open_file_directory: &str, save_file_path: &str, width: u16, height: u16, colour_map: &[u8]) -> Result<(), String> {
    use gif::{Frame, Encoder, Repeat};
    use std::fs::File;
    use std::borrow::Cow;

    let vecs : Vec<String> = helper::get_images_from_directory(open_file_directory);
    let mut image = File::create(save_file_path).unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, colour_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for vec in &vecs {
        let frame_bytes = open(vec.to_string()).unwrap();
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer =  Cow::Borrowed(frame_bytes.as_bytes());
        frame.delay = 30;
        frame.top = 0;
        frame.left = 0;
        encoder.write_frame(&frame).unwrap();
    }

    Ok(())
}

pub fn make_gif_alt(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    make_custom_gif(open_file_directory, save_file_path, 100, 100, &[0x32, 0x32, 0x32, 0x32, 0x32, 0x32])?;

    Ok(())
}
