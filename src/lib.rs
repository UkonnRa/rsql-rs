#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;

use crate::error::ParserError;

#[macro_use]
pub mod macros;
mod ast;
pub use ast::{comparison::*, constraint::*, expr::*, Operator};
pub mod error;
pub mod parser;

pub(crate) type ParserResult<T> = std::result::Result<T, ParserError>;

#[derive(Display, Debug)]
pub enum QueryType {
    Fiql,
    Rsql,
}
