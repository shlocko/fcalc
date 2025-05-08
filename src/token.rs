use alloc::string::String;

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
