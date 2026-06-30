#[cfg(test)]
pub mod makegifs_test {
    use assert_cmd::Command;
    use gif::streaming_decoder::OutputBuffer::Vec;
    use std::fmt::format;

    const APP_NAME: &str = "gifmaker";

    fn create_gifs(
        cmd_name: &str,
        open_directory: &str,
        save_file_path: &str,
    ) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg(cmd_name)
            .arg("--open-file-directory")
            .arg(open_directory)
            .arg("--save-gif-file-path")
            .arg(save_file_path);

        let output = cmd.unwrap();

        Ok(())
    }

    fn create_custom_gifs(
        cmd_name: &str,
        open_directory: &str,
        save_file_path: &str,
        width: &str,
        height: &str,
        color_map: &str,
    ) -> Result<(), String> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        cmd.arg(cmd_name)
            .arg("--open-file-directory")
            .arg(open_directory)
            .arg("--save-gif-file-path")
            .arg(save_file_path)
            .arg("--width")
            .arg(width)
            .arg("--height")
            .arg(height)
            .arg("--colour-map")
            .arg(color_map);

        let output = cmd.unwrap();

        Ok(())
    }

    #[test]
    fn create_countdown_gif() {
        println!("- create countdown gif: numbers.gif");
        let output = create_gifs("make-gif", "test-images/numbers", "test-output/numbers.gif");
    }

    #[test]
    fn create_alt() {
        println!("- create alt: custom.gif");
        let output = create_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom.gif",
        );
    }

    #[test]
    fn create_custom_one() {
        println!("- create custom one: custom_01.gif");
        let output = create_custom_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom_01.gif",
            "100",
            "100",
            "255, 255, 255, 0",
        );
    }

    #[test]
    fn create_custom_two() {
        println!("- create custom two: custom_02.gif");
        let output = create_custom_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom_02.gif",
            "100",
            "100",
            "100, 100, 100, 0, 0",
        );
    }

    #[test]
    fn create_custom_gif_as_0() {
        println!("- create custom one: custom_0.gif");
        let output = create_custom_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom_0.gif",
            "100",
            "100",
            "255, 255, 255, 0",
        );
    }

    #[test]
    fn create_custom_gif_as_255() {
        println!("- create custom one: custom_255.gif");
        let output = create_custom_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom_255.gif",
            "100",
            "100",
            "255, 255, 255, 255",
        );
    }

    #[test]
    fn create_custom_gif_as_wildcard() {
        println!("- create custom one: custom_wildcard.gif");
        let output = create_custom_gifs(
            "make-custom",
            "test-images/numbers",
            "test-output/custom_wildcard.gif",
            "100",
            "100",
            "255, 255, 255, 255,255, 255, 255, 255, 255, 255, 255, 255, 255,255, 255, 255, 255, 255, 255",
        );
    }

    #[test]
    fn create_custom_batch() {
        println!("- create custom batch:");

        for idx in 1..=255 {
            let default_val = 120;

            let colour_map = color_map_multiplier(format!("{}", default_val), 6);
            let file_name = format!(
                "test-output/custom_batch_{}_{}.gif",
                idx.to_string(),
                default_val
            );

            println!("- colour map: {}", colour_map);

            let output = create_custom_gifs(
                "make-custom",
                "test-images/numbers",
                file_name.as_str(),
                "100",
                "100",
                colour_map.as_str(),
            );
        }
    }

    fn color_map_multiplier(value: String, multiplier: i32) -> String {
        let mut text_value = "".to_string();
        for idx in 1..=multiplier {
            text_value.push_str(&value);
            text_value.push_str(",");
        }

        text_value[..text_value.len() - 1].to_string()
    }
}
