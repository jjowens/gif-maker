
pub mod a0_init_test {
    use std::fs;
    use assert_cmd::Command;

    fn delete_files()  -> std::io::Result<()> {
        println!("- delete files from output directory");
        fs::remove_dir_all("test-output")?;

        fs::create_dir("test-output")?;
        Ok(())
    }

}