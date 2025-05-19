use crate::{expression::Expression, Number};
use alloc::string::ToString;

pub fn simplify_expr(expr: Expression) -> Expression {
    match expr {
        Expression::Add { left, right } => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);
            match (&left, &right) {
                (Expression::Number(a, b), Expression::Number(c, d)) => {
                    return Expression::Number(a * d + c * b, b * d)
                }
                _ => todo!(),
            }
        }
        _ => expr,
    }
}
