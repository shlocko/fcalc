use crate::token::Token;
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
    Literal(String),
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Token,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Self::Literal(i) => i.to_string(),
            Self::Binary {
                left,
                right,
                operator,
            } => {
                let left = left.to_string();
                let right = right.to_string();
                format!("({:?} {left} {right})", operator)
            }
        }
    }
}
