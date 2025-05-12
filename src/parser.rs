use alloc::boxed::Box;
use alloc::vec;
use core::panic;

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression},
    Token,
};
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
    }
    matched
}

fn term(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Box<dyn Expression> {
    let left = primary(tokens, current, length);
    if match_next_token(
        tokens,
        current,
        vec![Token::Plus, Token::Minus, Token::Slash, Token::Star],
        length,
    ) {
        let op = previous(tokens, current);
        let right = primary(tokens, current, length);
        return Box::new(BinaryExpression {
            left_expression: left,
            operator: op,
            right_expression: right,
        });
    }

    left
}

fn primary(tokens: &Vec<Token>, current: &mut usize, length: usize) -> Box<dyn Expression> {
    let token = consume_token(tokens, current);
    match token {
        Token::Number(i) => Box::new(LiteralExpression { value: i }),
        _ => panic!("Invalid token"),
    }
}

pub fn parse(tokens: Vec<Token>) -> Box<dyn Expression> {
    let length: usize = tokens.len();
    let mut current: usize = 0;

    if !is_at_end(length, current) {
        term(&tokens, &mut current, length)
    } else {
        panic!("NO TOKENS(parse)");
    }
}
