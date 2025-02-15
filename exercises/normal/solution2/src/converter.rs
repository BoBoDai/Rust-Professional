pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let Some(left_index) = num_str.find("(")
    else { panic!("Failed to convert: place enter correct format") };
    let Some(right_index) = num_str.find(")")
    else { panic!("Failed to convert: place enter correct format") };
    let input_num = &num_str[..left_index];
    let from_base = &num_str[left_index + 1..right_index];
    let from_base = from_base.parse::<u32>()
        .expect("Failed to convert: place enter correct from base number");
    let mut mid_num = u32::from_str_radix(input_num, from_base)
        .expect("Failed to convert: place enter correct convert number");

    if mid_num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while mid_num > 0 {
        let remainder = mid_num % to_base;

        let digit = if remainder < 10 {
            (b'0' + (remainder as u8)) as char
        } else {
            (b'a' + (remainder - 10) as u8) as char
        };
        result.insert(0, digit);
        mid_num /= to_base;
    }
    result
}
