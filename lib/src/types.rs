use nom::{error::Error as NomError, Err as NomErr};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),

    Neg(Box<Expr>),
    Pos(Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
}

impl Clone for Expr {
    fn clone(&self) -> Expr {
        match self {
            Expr::Number(value) => Expr::Number(*value),

            Expr::Neg(expr) => Expr::Neg(Box::new(*expr.clone())),
            Expr::Pos(expr) => Expr::Pos(Box::new(*expr.clone())),

            Expr::Add(left, right) => Expr::Add(Box::new(*left.clone()), Box::new(*right.clone())),
            Expr::Sub(left, right) => Expr::Sub(Box::new(*left.clone()), Box::new(*right.clone())),
            Expr::Mul(left, right) => Expr::Mul(Box::new(*left.clone()), Box::new(*right.clone())),
            Expr::Div(left, right) => Expr::Div(Box::new(*left.clone()), Box::new(*right.clone())),
            Expr::Exp(left, right) => Expr::Exp(Box::new(*left.clone()), Box::new(*right.clone())),
        }
    }
}

pub type ParseError<'a> = NomErr<NomError<&'a str>>;
