/// Check a Luhn checksum.
use unicode_segmentation::UnicodeSegmentation;

fn does_code_contain_non_ascii(code: &Vec<&str>) -> bool {
    if code.into_iter().find(|s| !s.is_ascii()).is_some() {
        return true;
    }
    false
}

fn does_code_contain_non_digit(code: &Vec<char>) -> bool {
    if code
        .into_iter()
        .find(|c| !('0'..='9').contains(c))
        .is_some()
    {
        return true;
    }
    false
}

fn calculate_checksum(code: &Vec<u32>) -> u32 {
    let modulo_res = if code.len() % 2 == 0 { 0 } else { 1 };
    code.into_iter()
        .enumerate()
        .map(|(i, x)| if i % 2 == modulo_res { x * 2 } else { *x })
        .map(|x| if x > 9 { x - 9 } else { x })
        .sum()
}

fn preprocess(code: &str) -> Option<Vec<u32>> {
    let code_trimmed = code.trim();
    if code_trimmed.len() <= 1 {
        return None;
    }
    let code_without_spaces = code_trimmed.replace(" ", "");
    let new_code: Vec<&str> = code_without_spaces.graphemes(true).into_iter().collect();
    if does_code_contain_non_ascii(&new_code) {
        return None;
    }
    let new_ascii_code: Vec<char> = new_code
        .into_iter()
        .map(|s| s.chars().nth(0))
        .flatten()
        .collect();

    if does_code_contain_non_digit(&new_ascii_code) {
        return None;
    }

    let code_digits: Vec<u32> = new_ascii_code
        .into_iter()
        .map(|c| c as u8 - '0' as u8)
        .map(|x| x as u32)
        .collect();
    Some(code_digits)
}

pub fn is_valid(code: &str) -> bool {
    let code_digits;
    match preprocess(code) {
        Some(v) => code_digits = v,
        None => return false,
    }
    let checksum = calculate_checksum(&code_digits);
    checksum % 10 == 0
}
