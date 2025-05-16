use crate::{expression::Expression, Number};
use alloc::string::ToString;

fn simplify_expr(expr: Expression) -> Expression {
    match expr {
        Expression::Literal(n) => expr,
        Expression::Rational { left, right } => {
            let left_simplified = simplify_expr(left);
            let right_simplified = simplify_expr(right);
        }
        _ => {}
    }
}
