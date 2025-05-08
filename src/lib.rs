#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod scanner;
pub mod token;

pub use scanner::is_digit;
pub use scanner::scan;
pub use token::Token;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse(src: &str) -> Vec<Token> {
    scan(src.to_string())
}

#[cfg(test)]
mod tests {
    use scanner::number;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse() {
        let test_str = "123";
        assert_eq!(vec![Token::Number("123".to_string()),], parse(test_str));
    }

    #[test]
    fn test_digit() {
        let test_char: char = '1';
        assert_eq!(is_digit(test_char), true);
    }

    #[test]
    fn check() {
        assert_eq!(
            "1".to_string(),
            "123".to_string().chars().nth(0).unwrap().to_string()
        );
    }
}
