#[cfg(test)]
pub mod makegifs_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "gif_maker";

    fn create_gifs(open_directory: &str, save_file_path: &str) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg("make-gif")
            .arg("--open-file-directory").arg(open_directory)
            .arg("--save-gif-file-path").arg(save_file_path);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_countdown_gif() {
        let output = create_gifs("test-images/numbers", "test-output/numbers.gif");
    }

}