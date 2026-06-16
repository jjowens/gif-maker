pub fn create_dir_if_not_exists(directory_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(directory_path)?;
    Ok(())
}