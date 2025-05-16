#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod expression;
pub mod parser;
pub mod scanner;
pub mod simplify;
pub mod token;

use expression::Expression;
pub use parser::parse;
pub use scanner::is_digit;
pub use scanner::scan;
pub use token::{Number, Token};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn run(src: &str) -> Expression {
    let tokens = scan(src.to_string());
    parse(tokens)
}

#[cfg(test)]
mod tests {
    use expression::Expression;
    use parser::parse;
    use scanner::number;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_scan_number() {
        let test_str = "123/7+8";
        assert_eq!(
            vec![
                Token::Number(Number::Integer(123)),
                Token::Slash,
                Token::Number(Number::Integer(7)),
                Token::Plus,
                Token::Number(Number::Integer(8))
            ],
            scan(test_str.to_string())
        );
    }

    #[test]
    fn test_digit() {
        let test_char: char = '1';
        //assert!(is_digit(test_char));
        assert!(is_digit(test_char));
    }

    #[test]
    fn test_expr_to_string() {
        let expr = run("1*2+1");
        let expr_str = expr.to_string();
        println!("{}", expr_str);
        assert_eq!(expr_str, "(Plus (Star 1 2) 1)");
    }

    #[test]
    fn test_unary() {
        let expr = run("-1*-2");
        let expr_str = expr.to_string();
        println!("{}", expr_str);
        assert_eq!(expr_str, "(Star (Minus 1) (Minus 2))");
    }

    #[test]
    fn test_rational() {
        let expr = run("2+1/4");
        let expr_str = expr.to_string();
        println!("{expr_str}");
        assert_eq!(expr_str, "(Plus 2 (Rational 1/4))")
    }

    #[test]
    fn test_reduce() {
        let expr = run("(8/12)/3");
        let expr_str = expr.to_string();
    }
}
