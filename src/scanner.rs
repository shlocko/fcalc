use crate::token::Token;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

fn consume_char(chars: &Vec<char>, current: &mut usize) -> char {
    let cur_char = chars[*current];
    *current = *current + 1;
    cur_char
}

fn peek(chars: &Vec<char>, current: &usize) -> char {
    chars[*current]
}

fn is_at_end(len: usize, current: usize) -> bool {
    current >= len
}

pub fn is_digit(cur_char: char) -> bool {
    cur_char.is_digit(10)
}

pub fn number(cur_char: char, current: &mut usize, chars: &Vec<char>, length: usize) -> Token {
    let mut st: String = "".to_string();
    st.push_str(cur_char.to_string().as_str());
    while !is_at_end(length, *current) && is_digit(peek(chars, current)) {
        let ch = consume_char(chars, current);
        st.push_str(ch.to_string().as_str());
    }
    Token::Number(st)
}

pub fn scan(src: String) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let length = chars.len();
    let mut tokens: Vec<Token> = vec![];
    let mut current: usize = 0;

    while !is_at_end(length, current) {
        let cur_char: char = consume_char(&chars, &mut current);
        match cur_char {
            '(' => tokens.push(Token::RParen),
            ')' => tokens.push(Token::LParen),
            '/' => {
                if peek(&chars, &current) == '/' {
                    tokens.push(Token::SlashSlash);
                    consume_char(&chars, &mut current);
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '*' => tokens.push(Token::Star),
            '-' => tokens.push(Token::Minus),
            '+' => tokens.push(Token::Plus),
            _ => {
                if is_digit(cur_char) {
                    tokens.push(number(cur_char, &mut current, &chars, length));
                } else {
                    tokens.push(Token::Plus);
                }
            }
        }
    }

    tokens
}
