use crate::{evaluator::evaluate, parser::parse, types::ParseError};

pub fn run(expr: &str) -> Result<f64, ParseError> {
    let parsed = parse(expr)?;
    Ok(evaluate(parsed))
}
