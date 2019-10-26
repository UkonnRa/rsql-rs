#![feature(backtrace)]

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;

use crate::error::ParserError;

pub mod ast;
pub mod error;
pub mod parser;

type ParserResult<T> = std::result::Result<T, ParserError>;

#[derive(Display, Debug)]
pub enum QueryType {
    Fiql,
    Rsql,
}
