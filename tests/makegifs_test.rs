#[cfg(test)]
pub mod makegifs_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "gif_maker";

    fn create_gifs(cmd_name: &str, open_directory: &str, save_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg(cmd_name)
            .arg("--open-file-directory").arg(open_directory)
            .arg("--save-gif-file-path").arg(save_file_path);

        let output = cmd.unwrap();

        Ok(())
    }

    fn create_custom_gifs(cmd_name: &str, open_directory: &str, save_file_path: &str, width: &str, height: &str, color_map: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg(cmd_name)
            .arg("--open-file-directory").arg(open_directory)
            .arg("--save-gif-file-path").arg(save_file_path)
            .arg("--save-gif-file-path").arg(save_file_path)
            // .arg("--width").arg(width)
            // .arg("--height").arg(height)
            .arg("--colour-map").arg(color_map);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_countdown_gif() {
        let output = create_gifs("make-gif","test-images/numbers", "test-output/numbers.gif");
    }

    #[test]
    fn create_alt() {
        let output = create_gifs("make-custom","test-images/numbers", "test-output/custom.gif");
    }

    #[test]
    fn create_custom_one() {
        let output = create_custom_gifs("make-custom",
                                        "test-images/numbers",
                                        "test-output/custom_01.gif",
                                        "100",
                                        "100",
                                         "0x32, 0x32, 0x32, 0x32, 0x32, 0x32");
    }

}