use alloc::string::String;

use crate::token::Token;
use alloc::format;

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

pub struct BinaryExpression<'a> {
    pub right_expression: &'a dyn Expression,
    pub left_expression: &'a dyn Expression,
    pub operator: Token,
}

impl Expression for BinaryExpression<'_> {
    fn to_string(&self) -> String {
        let left = self.right_expression.to_string();
        let right = self.left_expression.to_string();
        format!("{left}  {right}")
    }
}
