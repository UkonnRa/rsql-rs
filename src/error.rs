use crate::QueryType;
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid Pair Rule found")]
    InvalidPairRule(Backtrace),

    #[error("Invalid Comparison found: {0}")]
    InvalidComparison(String),
    #[error("Invalid Query found: {0}")]
    InvalidQuery(QueryType, Backtrace),

    #[error("Invalid Constraint arguments: expect: {0}, found: {1}")]
    InvalidConstraintArgs(String, usize),
    #[error("Cannot find {field} when constructing {ty}")]
    LackOfField { ty: String, field: String },

    #[error("Unhandled Error: {0}")]
    Unhandled(#[source] anyhow::Error),
}

impl From<anyhow::Error> for ParserError {
    fn from(err: anyhow::Error) -> Self {
        match err.downcast::<ParserError>() {
            Ok(par_err) => par_err,
            Err(any_err) => ParserError::Unhandled(any_err),
        }
    }
}
