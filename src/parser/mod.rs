use crate::ast::comparison::Comparison;
use crate::ast::constraint::Constraint;
use crate::ast::node::Node;
use crate::ParserResult;
use pest::iterators::Pair;
use pest::RuleType;
use std::collections::HashMap;

pub mod fiql;

pub trait Parser {
    type R: RuleType;

    fn parse_to_node(code: &str) -> ParserResult<Box<dyn Node>>;

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison>;
}
