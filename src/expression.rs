use crate::token::{Number, Token};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};

pub enum Expression {
    Number(i32),
    Float(f32),
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Subtract {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Divide {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Rational {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(i) => i.to_string(),
            Self::Float(i) => i.to_string(),
            Self::Add { left, right } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("(Add {left} {right})")
            }
            Self::Subtract { left, right } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("(Subtract {left} {right})")
            }
            Self::Multiply { left, right } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("(Multiply {left} {right})")
            }
            Self::Divide { left, right } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("(FloatDivide {left}//{right})")
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
            Self::Grouping { expr } => {
                let expr = expr.to_string();
                format!("(Grouping {expr})")
            }
        }
    }
}
