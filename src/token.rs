use alloc::fmt::Debug;
use alloc::string::String;
use core::write;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,
    LParen,
    RParen,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
    Rational(i32, i32),
    Float(f32),
}
