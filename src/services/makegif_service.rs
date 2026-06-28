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


pub fn make_gif_alt_backup(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    use gif::{Frame, Encoder, Repeat};
    use std::fs::File;
    use std::borrow::Cow;

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let (width, height) = (6, 6);
    let beacon_states = [[
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ], [
        0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 0,
        0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
    ]];
    let mut image = File::create("target/beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for state in &beacon_states {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
    Ok(())
}

pub fn make_gif_256_colours(open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    use std::fs::File;

    // Get pixel data from some source
    let mut pixels: Vec<u8> = vec![0; 30_000];
    // Create frame from data
    let frame = gif::Frame::from_rgb(100, 100, &mut *pixels);
    // Create encoder
    let mut image = File::create("target/indexed_color.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[]).unwrap();
    // Write frame to file
    encoder.write_frame(&frame).unwrap();
    Ok(())
}
