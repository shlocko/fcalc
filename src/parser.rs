use alloc::boxed::Box;
use alloc::vec;
use core::panic;

use crate::expression;
use crate::expression::Expression;
use crate::Number;
use crate::Token;
use alloc::vec::Vec;

fn is_at_end(length: usize, current: usize) -> bool {
    current >= length
}

fn consume_token(tokens: &Vec<Token>, current: &mut usize) -> Token {
    let token = tokens[*current].clone();
    *current = *current + 1;
    token
}

fn previous(tokens: &Vec<Token>, current: &usize) -> Token {
    tokens[current - 1].clone()
}

fn match_next_token(
    tokens: &Vec<Token>,
    current: &mut usize,
    to_match: Vec<Token>,
    length: usize,
) -> bool {
    let mut matched = false;
    if !is_at_end(length, *current) {
        let next_token = consume_token(tokens, current);
        for token in to_match {
            if token == next_token {
                matched = true;
            }
        }
        if !matched {
            *current = *current - 1
        }
    }
    matched
}

fn expression(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    term(tokens, current, length)
}

fn term(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    let left = factor(tokens, current, length);
    if match_next_token(tokens, current, vec![Token::Plus, Token::Minus], length) {
        let op = previous(tokens, current);
        let right = factor(tokens, current, length);
        let expr = match op {
            Token::Plus => Expression::Add {
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::Minus => Expression::Subtract {
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => panic!("Invalid operator for factor."),
        };
        return expr;
    }

    left
}

fn factor(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    let left = rational(tokens, current, length);
    if match_next_token(
        tokens,
        current,
        vec![Token::Star, Token::SlashSlash],
        length,
    ) {
        let op = previous(tokens, current);
        let right = rational(tokens, current, length);
        let expr = match op {
            Token::Star => Expression::Multiply {
                left: Box::new(left),
                right: Box::new(right),
            },
            Token::SlashSlash => Expression::Divide {
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => panic!("Invalid operator for factor."),
        };
        return expr;
    }

    left
}

fn rational(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    let left = unary(tokens, current, length);
    if match_next_token(tokens, current, vec![Token::Slash], length) {
        let right = unary(tokens, current, length);
        return Expression::Rational {
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    left
}

fn unary(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    if match_next_token(tokens, current, vec![Token::Minus, Token::Bang], length) {
        let op = previous(tokens, current);
        let right = unary(tokens, current, length);
        return Expression::Unary {
            operator: op,
            right: Box::new(right),
        };
    }
    return primary(tokens, current, length);
}

fn primary(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    let token = consume_token(tokens, current);
    match token {
        Token::Number(i) => match i {
            Number::Float(i) => Expression::Float(i),
            Number::Integer(i) => Expression::Number(i),
        },
        Token::LParen => {
            let expr = expression(tokens, current, length);
            let grp_expr = Expression::Grouping {
                expr: Box::new(expr),
            };
            if match_next_token(tokens, current, vec![Token::RParen], length) {
                return grp_expr;
            } else {
                panic!("Expected ')' after grouping.");
            }
        }
        _ => panic!("Invalid token {:?}", token),
    }
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let length: usize = tokens.len();
    let mut current: usize = 0;

    if !is_at_end(length, current) {
        expression(&tokens, &mut current, length)
    } else {
        panic!("NO TOKENS(parse)");
    }
}
