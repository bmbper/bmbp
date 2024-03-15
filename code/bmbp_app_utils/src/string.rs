use bmbp_app_common::BmbpHashMap;

/// BmbpStringUtil 字符串工具类

pub fn snake_to_camel(snake_string: String) -> String {
    case_style::CaseStyle::from_snakecase(snake_string).to_camelcase()
}

pub fn camel_to_snake(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
}

pub fn camel_to_snake_upper(camel_string: String) -> String {
    case_style::CaseStyle::from_camelcase(camel_string)
        .to_snakecase()
        .to_string()
        .to_uppercase()
}

pub fn zero_fill_left(number: i32, len: i32) -> String {
    return char_fill_left('0', number, len);
}

pub fn zero_fill_right(number: i32, len: i32) -> String {
    return char_fill_right('0', number, len);
}

pub fn char_fill_left(ch: char, number: i32, len: i32) -> String {
    let number_str = number.to_string();
    let left_zero: i32 = len - (number_str.len() as i32);
    if left_zero <= 0 {
        return number_str;
    }
    let mut zero_char: Vec<char> = vec![ch; left_zero as usize];
    let mut number_str_char: Vec<char> = number_str.chars().collect();
    zero_char.append(&mut number_str_char);
    return zero_char.iter().collect();
}

pub fn char_fill_right(ch: char, number: i32, len: i32) -> String {
    let number_str = number.to_string();
    let left_zero: i32 = len - (number_str.len() as i32);
    if left_zero <= 0 {
        return number_str;
    }
    let mut zero_char: Vec<char> = vec![ch; left_zero as usize];
    let mut number_str_char: Vec<char> = number_str.chars().collect();
    number_str_char.append(&mut zero_char);
    return number_str_char.iter().collect();
}

pub fn render_to_json_string(value: &BmbpHashMap) -> String {
    serde_json::to_string_pretty(value).unwrap()
}

pub fn is_empty_string<T>(value: Option<T>) -> bool
where
    T: ToString,
{
    if value.is_none() {
        return true;
    }
    value.as_ref().unwrap().to_string().is_empty()
}
