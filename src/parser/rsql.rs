use crate::Arguments;
use crate::Comparison;

use crate::error::ParserError;
use crate::parser::Parser;
use pest::iterators::Pair;
use pest::Parser as PestParser;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Parser, Default)]
#[grammar = "rsql.pest"]
pub struct RsqlParser(HashMap<String, Comparison>);
gen_parser!(RsqlParser);

impl Parser for RsqlParser {
    type R = Rule;

    gen_basic_parser!(RSQL);
}

impl<'i> TryFrom<Pair<'i, Rule>> for Arguments {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::argument => {
                let mut args = vec![];
                for arg_item in value.into_inner() {
                    let item_rules = arg_item.as_rule();
                    let args_inner = arg_item.into_inner();
                    match item_rules {
                        Rule::value => {
                            for arg_inner in args_inner {
                                match arg_inner.as_rule() {
                                    Rule::unreserved_str => {
                                        for unreserved_inner in arg_inner.into_inner() {
                                            if unreserved_inner.as_rule() == Rule::unreserved_inner
                                            {
                                                args.push(unreserved_inner.as_str().to_string());
                                            }
                                        }
                                    }
                                    Rule::double_quoted => {
                                        for double_inner in arg_inner.into_inner() {
                                            if double_inner.as_rule() == Rule::double_quoted_inner {
                                                args.push(double_inner.as_str().to_string());
                                            }
                                        }
                                    }
                                    Rule::single_quoted => {
                                        for single_inner in arg_inner.into_inner() {
                                            if single_inner.as_rule() == Rule::single_quoted_inner {
                                                args.push(single_inner.as_str().to_string());
                                            }
                                        }
                                    }
                                    _ => ParserError::invalid_pair_rule()?,
                                }
                            }
                        }
                        _ => ParserError::invalid_pair_rule()?,
                    }
                }

                Ok(Arguments(args))
            }
            _ => ParserError::invalid_pair_rule()?,
        }
    }
}
