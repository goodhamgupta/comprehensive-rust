#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    const RADIX: u32 = 10;
    let numbers: String = cc_number.chars().filter(|c| !c.is_whitespace()).collect();
    if numbers.chars().all(char::is_alphabetic) {
        false
    } else if numbers.len() <= 2 {
        false
    } else if numbers.chars().all(char::is_numeric) {
        // cc_number only contains numeric values
        let mut iter = numbers.chars().rev().into_iter();
        let mut even_digit_sum = 0;
        let mut odd_digit_sum = 0;
        for (idx, c) in iter.enumerate() {
            let digit = c.to_digit(RADIX).unwrap();
            if idx % 2 == 0 {
                odd_digit_sum += digit;
            } else {
                let digit_sum: u32 = (digit * 2)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(RADIX).unwrap())
                    .sum();
                even_digit_sum += digit_sum;
            }
        }
        let total = even_digit_sum + odd_digit_sum;
        if total % 10 == 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("   "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(!luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 9569"));
}

fn main() {}
