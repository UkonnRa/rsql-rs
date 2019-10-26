use crate::ast::comparison::Comparison;
use crate::ast::expr::Expr;
use crate::ParserResult;
use pest::RuleType;
use std::collections::HashMap;

pub mod fiql;

pub trait Parser {
    type R: RuleType;

    fn parse_to_node(code: &str) -> ParserResult<Expr>;

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison>;
}
