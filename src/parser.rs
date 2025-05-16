use alloc::boxed::Box;
use alloc::vec;
use core::panic;

use crate::expression::Expression;
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

fn term(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Expression {
    let left = factor(tokens, current, length);
    if match_next_token(tokens, current, vec![Token::Plus, Token::Minus], length) {
        let op = previous(tokens, current);
        let right = factor(tokens, current, length);
        return Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
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
        return Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
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
        Token::Number(i) => Expression::Literal(i),
        _ => panic!("Invalid token"),
    }
}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let length: usize = tokens.len();
    let mut current: usize = 0;

    if !is_at_end(length, current) {
        term(&tokens, &mut current, length)
    } else {
        panic!("NO TOKENS(parse)");
    }
}
