use alloc::fmt::Debug;
use alloc::string::String;
use core::write;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,
    LParen,
    RParen,
}
