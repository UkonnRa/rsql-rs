use crate::{ParserResult, QueryType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Empty symbol array when creating a new Comparison")]
    EmptySymbol(),

    #[error("Invalid Pair Rule found")]
    InvalidPairRule(),

    #[error("Invalid Comparison found: {0}")]
    InvalidComparison(String),
    #[error("Invalid Query found: {0}")]
    InvalidQuery(QueryType),

    #[error("Invalid Constraint arguments: expect: {0}, found: {1}")]
    InvalidConstraintArgs(String, usize),
    #[error("Cannot find {field} when constructing {ty}")]
    LackOfField { ty: String, field: String },

    #[error("Unhandled Error: {0}")]
    Unhandled(#[source] anyhow::Error),
}

impl ParserError {
    pub fn invalid_pair_rule<T>() -> ParserResult<T> {
        Err(ParserError::InvalidPairRule())
    }
}

impl From<anyhow::Error> for ParserError {
    fn from(err: anyhow::Error) -> Self {
        match err.downcast::<ParserError>() {
            Ok(par_err) => par_err,
            Err(any_err) => ParserError::Unhandled(any_err),
        }
    }
}

impl From<pest::error::Error<crate::parser::fiql::Rule>> for ParserError {
    fn from(err: pest::error::Error<crate::parser::fiql::Rule>) -> Self {
        ParserError::Unhandled(anyhow::Error::from(err))
    }
}

impl From<pest::error::Error<crate::parser::rsql::Rule>> for ParserError {
    fn from(err: pest::error::Error<crate::parser::rsql::Rule>) -> Self {
        ParserError::Unhandled(anyhow::Error::from(err))
    }
}
