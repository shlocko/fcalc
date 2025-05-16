use crate::token::{Number, Token};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};

pub enum Expression {
    Literal(Number),
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Token,
    },
    Rational {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Self::Literal(i) => match i {
                Number::Integer(i) => i.to_string(),
                Number::Float(i) => i.to_string(),
            },
            Self::Binary {
                left,
                right,
                operator,
            } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("({:?} {left} {right})", operator)
            }
            Self::Rational { left, right } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("(Rational {left}/{right})")
            }
            Self::Unary { operator, right } => {
                let right = right.to_string();
                format!("({:?} {right})", operator)
            }
        }
    }
}
