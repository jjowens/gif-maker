#[cfg(test)]
pub mod makegifs_256_colours_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "gifmaker";

    fn create_256_colours(cmd_name: &str, open_directory: &str, save_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg(cmd_name)
            .arg("--open-file-directory").arg(open_directory)
            .arg("--save-gif-file-path").arg(save_file_path);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_256_colours_01() {
        println!("- create 256 colours: 256_colours_01.gif");
        let output = create_256_colours("make256",
                                        "test-images/numbers",
                                        "test-output/256_colours_01.gif");
    }

}