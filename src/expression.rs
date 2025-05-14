use crate::token::{Number, Token};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};

/*pub trait Expression {
    fn to_string(&self) -> String;
    fn simplify(&self) -> Expression;
}

pub struct LiteralExpression {
    pub value: String,
}

impl Expression for LiteralExpression {
    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn simplify(&self) -> Expression {
        self
    }
}

pub struct BinaryExpression {
    pub right_expression: Box<dyn Expression>,
    pub left_expression: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for BinaryExpression {
    fn to_string(&self) -> String {
        let left = self.left_expression.to_string();
        let right = self.right_expression.to_string();
        let op = &self.operator;
        format!("{left} {:?} {right}", op)
    }
    fn simplify(&self) -> Expression {
        self
    }
}*/
pub enum Expression {
    Literal(Number),
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Token,
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
                Number::Rational(n, d) => format!("{n}/{d}"),
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
            Self::Unary { operator, right } => {
                let right = right.to_string();
                format!("({:?} {right})", operator)
            }
        }
    }
}
