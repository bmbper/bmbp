/// BmbpStringUtil 字符串工具类

pub fn snake_to_camel(snake_string: String) -> String {
    let snake_char_vec: Vec<char> = snake_string.to_ascii_lowercase().chars().collect();
    let mut camel_char_vec: Vec<char> = Vec::new();
    let mut is_to_upper: bool = false;
    for ch in snake_char_vec {
        if ch == '_' {
            is_to_upper = true;
            continue;
        }
        if is_to_upper {
            camel_char_vec.push(ch.to_ascii_uppercase());
            is_to_upper = false;
        } else {
            camel_char_vec.push(ch);
        }
    }
    return camel_char_vec.iter().collect();
}
pub fn camel_to_snake(camel_string: String) -> String {
    let camel_char_vec: Vec<char> = camel_string.chars().collect();
    let mut snake_char_vec: Vec<char> = Vec::new();
    for ch in camel_char_vec {
        if ch == '_' {
            continue;
        }
        if ch != ch.to_ascii_lowercase() {
            snake_char_vec.push('_');
            snake_char_vec.push(ch.to_ascii_lowercase())
        } else {
            snake_char_vec.push(ch);
        }
    }
    return snake_char_vec.iter().collect();
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
