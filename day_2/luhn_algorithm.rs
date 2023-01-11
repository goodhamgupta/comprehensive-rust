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
                let digit_sum: u32 = (digit * 2)
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(RADIX).unwrap())
                    .sum();
                even_digit_sum += digit_sum;
            } else {
                println!("odd: {odd_digit_sum}");
                odd_digit_sum += digit;
            }
        }
        let total = even_digit_sum + odd_digit_sum;
        println!("even: {even_digit_sum}");
        println!("odd: {odd_digit_sum}");
        println!("total: {total}");
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
    assert!(luhn("4263 9826 4026 9299"))
}

fn main() {}
