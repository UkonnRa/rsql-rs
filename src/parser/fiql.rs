use crate::ast::comparison::Comparison;
use crate::ast::constraint::{Arguments};
use crate::ast::expr::Expr;
use crate::ast::{comparison};
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
        ]);
        let tuple_vec: Vec<(&'static str, &'static Comparison)> = tuple_vec
            .iter()
            .flat_map(|&comp| comp.symbols.iter().map(move |sym| (sym.as_str(), comp)))
            .collect();
        tuple_vec.into_iter().collect()
    };
}

#[derive(Parser)]
#[grammar = "fiql.pest"]
pub struct FiqlParser;
gen_parser!(FiqlParser);

impl Parser for FiqlParser {
    type R = Rule;

    gen_parser_to_node!();

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison> {
        &DEFAULT_COMPS_MAP as &'static HashMap<&'static str, &'static Comparison>
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Arguments {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let arg = value.as_str();
        Ok(Arguments(vec![arg.to_string()]))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::fiql::*;
    use crate::parser::Parser;
    use crate::ParserResult;

    #[test]
    fn default_fiql_map_test() -> ParserResult<()> {
        let _ = env_logger::builder().is_test(true).try_init();
        let _ = FiqlParser::default_comparisons();
        Ok(())
    }
}
