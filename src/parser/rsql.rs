use crate::ast::comparison;
use crate::ast::comparison::Comparison;
use crate::ast::constraint::Arguments;
use crate::ast::expr::Expr;
use crate::error::ParserError;

use crate::parser::Parser;
use crate::ParserResult;
use pest::iterators::Pair;
use pest::Parser as PestParser;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;

lazy_static! {
    pub(crate) static ref DEFAULT_COMPS_MAP: HashMap<&'static str, &'static Comparison> = {
        let tuple_vec: HashSet<&'static Comparison> = HashSet::from_iter(vec![
            &comparison::EQUAL as &'static Comparison,
            &comparison::NOT_EQUAL as &'static Comparison,
            &comparison::GREATER_THAN as &'static Comparison,
            &comparison::GREATER_THAN_OR_EQUAL as &'static Comparison,
            &comparison::LESS_THAN as &'static Comparison,
            &comparison::LESS_THAN_OR_EQUAL as &'static Comparison,
            &comparison::IN as &'static Comparison,
            &comparison::OUT as &'static Comparison,
        ]);
        let tuple_vec: Vec<(&'static str, &'static Comparison)> = tuple_vec
            .iter()
            .flat_map(|&comp| comp.symbols.iter().map(move |sym| (sym.as_str(), comp)))
            .collect();
        tuple_vec.into_iter().collect()
    };
}

#[derive(Parser)]
#[grammar = "rsql.pest"]
pub struct RsqlParser;
gen_parser!(RsqlParser);

impl Parser for RsqlParser {
    type R = Rule;

    gen_parser_to_node!();

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison> {
        &DEFAULT_COMPS_MAP as &'static HashMap<&'static str, &'static Comparison>
    }
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
                                    },
                                    Rule::double_quoted => {
                                        for double_inner in arg_inner.into_inner() {
                                            if double_inner.as_rule() == Rule::double_quoted_inner {
                                                args.push(double_inner.as_str().to_string());
                                            }
                                        }
                                    },
                                    Rule::single_quoted => {
                                        for single_inner in arg_inner.into_inner() {
                                            if single_inner.as_rule() == Rule::single_quoted_inner {
                                                args.push(single_inner.as_str().to_string());
                                            }
                                        }
                                    },
                                    _ => ParserError::invalid_pair_rule()?,
                                }
                            }
                        },
                        _ => ParserError::invalid_pair_rule()?,
                    }
                }

                Ok(Arguments(args))
            },
            _ => ParserError::invalid_pair_rule()?,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::rsql::*;
    use crate::parser::Parser;
    use crate::ParserResult;

    #[test]
    fn default_rsql_map_test() -> ParserResult<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let _ = RsqlParser::default_comparisons();
        Ok(())
    }
}
