pub fn create_dir_if_not_exists(directory_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(directory_path)?;
    Ok(())
}

pub fn get_images_from_directory(open_file_directory: &str) -> Vec<String> {
    let mut vecs  = vec![];

    vecs.push("test-images/numbers/countdown-01.png".to_string());
    vecs.push("test-images/numbers/countdown-02.png".to_string());
    vecs.push("test-images/numbers/countdown-03.png".to_string());
    vecs.push("test-images/numbers/countdown-04.png".to_string());
    vecs.push("test-images/numbers/countdown-05.png".to_string());

    vecs
}