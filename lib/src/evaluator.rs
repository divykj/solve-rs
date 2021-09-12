use super::types::Expr;

pub fn evaluate(expr: Expr) -> f64 {
    match expr {
        Expr::Number(value) => value,

        Expr::Neg(expr) => -evaluate(*expr),
        Expr::Pos(expr) => evaluate(*expr),

        Expr::Add(left, right) => evaluate(*left) + evaluate(*right),
        Expr::Sub(left, right) => evaluate(*left) - evaluate(*right),
        Expr::Mul(left, right) => evaluate(*left) * evaluate(*right),
        Expr::Div(left, right) => evaluate(*left) / evaluate(*right),
        Expr::Exp(left, right) => evaluate(*left).powf(evaluate(*right)),
    }
}
