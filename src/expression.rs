use alloc::string::String;

use crate::operator::Operator;

trait Expression {
    fn to_string() -> String;
}

pub struct LiteralExpression {
    value: String,
}

pub struct BinaryExpression {
    right_expression: Expression,
    left_expression: Expression,
    operator: Operator,
}
