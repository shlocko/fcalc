use crate::token::Token;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

pub trait Expression {
    fn to_string(&self) -> String;
}

pub struct LiteralExpression {
    pub value: String,
}

impl Expression for LiteralExpression {
    fn to_string(&self) -> String {
        self.value.clone()
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
}
