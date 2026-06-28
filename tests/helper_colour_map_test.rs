#[cfg(test)]
pub mod helper_colour_map_test {
    use assert_cmd::Command;
    use gifmaker::services::helper::split_string_into_colour_map;

    #[test]
    fn test_split_string_into_colour_map() {
        println!("- test_split_string_into_colour_map");
        let result = split_string_into_colour_map("0x32,0x32,0x32,0x32,0x32,0x32");

        assert_eq!(&[0x32, 0x32, 0x32, 0x32, 0x32, 0x32], result)
    }

}