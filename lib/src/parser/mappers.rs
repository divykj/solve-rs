use super::constants::{
    ADD_OPERATOR, DIVIDE_OPERATOR, EXPONENT_OPERATOR, MULTIPLY_OPERATOR, SUBTRACT_OPERATOR,
    UNARY_MINUS_OPERATOR, UNARY_PLUS_OPERATOR,
};
use crate::types::Expr;

pub(crate) fn fold_binary_operator(left: Expr, right_ops: Vec<(&str, Expr)>) -> Result<Expr, ()> {
    right_ops
        .into_iter()
        .try_fold(left, |left, (operator, right)| {
            map_binary_operator(operator, left, right)
        })
}

pub(crate) fn map_binary_operator(operator: &str, left: Expr, right: Expr) -> Result<Expr, ()> {
    Ok(match operator {
        ADD_OPERATOR => Expr::Add(Box::new(left), Box::new(right)),
        SUBTRACT_OPERATOR => Expr::Sub(Box::new(left), Box::new(right)),
        MULTIPLY_OPERATOR => Expr::Mul(Box::new(left), Box::new(right)),
        DIVIDE_OPERATOR => Expr::Div(Box::new(left), Box::new(right)),
        EXPONENT_OPERATOR => Expr::Exp(Box::new(left), Box::new(right)),
        _ => return Err(()),
    })
}

pub(crate) fn map_unary_operator(operator: &str, value: Expr) -> Result<Expr, ()> {
    Ok(match operator {
        UNARY_MINUS_OPERATOR => Expr::Neg(Box::new(value)),
        UNARY_PLUS_OPERATOR => Expr::Pos(Box::new(value)),
        _ => return Err(()),
    })
}

pub(crate) fn map_number(value: f64) -> Result<Expr, ()> {
    Ok(Expr::Number(value))
}
