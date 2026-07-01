pub fn get_images_from_directory(_open_file_directory: &str) -> Vec<String> {
    let mut vecs  = vec![];

    vecs.push("test-images/numbers/countdown-01.png".to_string());
    vecs.push("test-images/numbers/countdown-02.png".to_string());
    vecs.push("test-images/numbers/countdown-03.png".to_string());
    vecs.push("test-images/numbers/countdown-04.png".to_string());
    vecs.push("test-images/numbers/countdown-05.png".to_string());
    vecs.push("test-images/numbers/countdown-05a.png".to_string());

    vecs
}

pub fn split_string_into_colour_map(val: &str) -> Vec<i32> {
    //let result = val.split(",");
    let split_container = val.split(",");

    //let mut result = vec![];
    let mut result : Vec<i32> = vec![];

    for elem in split_container {
        result.push(elem.parse::<i32>().unwrap());
    }

    result.to_vec()
}

pub fn split_string_into_colour_map_as_u8(val: &str) -> Vec<u8> {
    //let result = val.split(",");
    let clean_value = val.replace(" ", "");
    let split_container = clean_value.split(",");

    //let mut result = vec![];
    let mut result : Vec<u8> = vec![];

    for elem in split_container {
        result.push(elem.parse::<u8>().unwrap());
    }

    result.to_vec()
}