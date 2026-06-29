pub fn get_images_from_directory(_open_file_directory: &str) -> Vec<String> {
    let mut vecs  = vec![];

    vecs.push("test-images/numbers/countdown-01.png".to_string());
    vecs.push("test-images/numbers/countdown-02.png".to_string());
    vecs.push("test-images/numbers/countdown-03.png".to_string());
    vecs.push("test-images/numbers/countdown-04.png".to_string());
    vecs.push("test-images/numbers/countdown-05.png".to_string());

    vecs
}

pub fn split_string_into_colour_map(val: &str) -> &[u8] {
    //let result = val.split(",").collect::<Vec<&[u8]>>();
    let split_container = val.split(",");

    let mut result = vec![];

    for elem in split_container {
        result.push(elem.as_bytes());
    }

    //&[0x32, 0x32, 0x32, 0x32, 0x32, 0x32]
    &[0xFF,0xFF,0xFF,0,0,0]
}