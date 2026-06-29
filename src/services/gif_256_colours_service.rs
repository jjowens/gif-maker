pub fn make_beacon(_open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
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
    let mut image = File::create(save_file_path).unwrap();
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

pub fn make_gif_256_colours(_open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    use std::fs::File;

    // Get pixel data from some source
    let mut pixels: Vec<u8> = vec![0; 30_000];
    // Create frame from data
    let frame = gif::Frame::from_rgb(100, 100, &mut *pixels);
    // Create encoder
    let mut image = File::create(save_file_path).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[]).unwrap();
    // Write frame to file
    encoder.write_frame(&frame).unwrap();
    Ok(())
}

pub fn make_gif_256_colours_alt(_open_file_directory: &str, save_file_path: &str) -> Result<(), String> {
    use std::fs::File;

    // Get pixel data from some source
    let mut pixels: Vec<u8> = vec![0; 30_000];
    // Create frame from data
    let frame = gif::Frame::from_rgb(100, 100, &mut *pixels);
    // Create encoder
    let mut image = File::create(save_file_path).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[]).unwrap();
    // Write frame to file
    encoder.write_frame(&frame).unwrap();
    Ok(())
}