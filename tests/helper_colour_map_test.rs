#[cfg(test)]
pub mod helper_colour_map_test {
    use gifmaker::services::helper::{split_string_into_colour_map, split_string_into_colour_map_as_u8};

    #[test]
    fn test_split_string_into_colour_map_01() {
        println!("- test_split_string_into_colour_map");
        let expected_result = vec![50, 50, 50, 50, 50, 50];
        let result = split_string_into_colour_map("50,50,50,50,50,50");

        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_split_string_into_colour_map_02() {
        println!("- test_split_string_into_colour_map");
        //let expected_result = vec![0xFF, 0xFF, 0xFF, 0, 0, 0];
        let expected_result = vec![255,255, 255, 0, 0, 0];
        let result = split_string_into_colour_map("255,255,255,0,0,0");

        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_split_string_into_colour_map_as_u8_03() {
        println!("- test_split_string_into_colour_map");
        let expected_result = vec![255, 255, 255, 0, 0, 0];
        let result = split_string_into_colour_map_as_u8("255,255,255,0,0,0");

        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_split_string_into_colour_map_as_u8_04() {
        println!("- test_split_string_into_colour_map");
        let expected_result = vec![50, 50, 50, 0, 0, 0];
        let result = split_string_into_colour_map_as_u8("50,50,50,0,0,0");

        assert_eq!(expected_result, result)
    }

    #[test]
    fn test_split_string_with_spaces_into_colour_map_as_u8() {
        println!("- test_split_string_into_colour_map");
        let expected_result = vec![255, 255, 255];
        let result = split_string_into_colour_map_as_u8(" 255, 255, 255 ");

        assert_eq!(expected_result, result)
    }

}