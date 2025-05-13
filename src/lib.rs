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
pub use token::Token;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn run(src: &str) -> Box<dyn Expression> {
    let tokens = scan(src.to_string());
    parse(tokens)
}

#[cfg(test)]
mod tests {
    use expression::{BinaryExpression, Expression, LiteralExpression};
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
                Token::Number("123".to_string()),
                Token::Slash,
                Token::Number("7".to_string()),
                Token::Plus,
                Token::Number("8".to_string())
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
        let rlit_expr = LiteralExpression {
            value: "Test".to_string(),
        };

        let llit_expr = LiteralExpression {
            value: "Test".to_string(),
        };
        let _bin_expr = BinaryExpression {
            right_expression: Box::new(rlit_expr),
            left_expression: Box::new(llit_expr),
            operator: Token::Plus,
        };
        let expr = run("1/2");
        let expr_str = expr.to_string();
        println!("{}", expr_str);
        assert_eq!(expr_str, "1 Plus 2");
    }
}
