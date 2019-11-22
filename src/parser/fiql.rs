use crate::Arguments;
use crate::Comparison;

use crate::error::ParserError;
use crate::parser::Parser;

use pest::iterators::Pair;
use pest::Parser as PestParser;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Parser, Default)]
#[grammar = "fiql.pest"]
pub struct FiqlParser(HashMap<String, Comparison>);
gen_parser!(FiqlParser);

impl Parser for FiqlParser {
    type R = Rule;

    gen_basic_parser!(FIQL);
}

impl<'i> TryFrom<Pair<'i, Rule>> for Arguments {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let arg = value.as_str();
        Ok(Arguments(vec![arg.to_string()]))
    }
}
