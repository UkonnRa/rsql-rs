use crate::Constraint;
use crate::Expr;
use crate::ParserResult;
use crate::{Comparison, Operator};
use pest::iterators::Pair;
use pest::RuleType;
use std::collections::HashMap;

pub mod fiql;
pub mod rsql;

pub trait Parser {
    type R: RuleType;

    fn get_inner_mut(&mut self) -> &mut HashMap<String, Comparison>;
    fn get_inner(&self) -> &HashMap<String, Comparison>;
    fn register_comparison(&mut self, comparison: &Comparison);
    fn remove_comparison_by_symbol(&mut self, symbol: &str);
    fn get_comparison(&self, symbol: &str) -> Option<Comparison>;

    fn parse_to_node(&self, code: &str) -> ParserResult<Expr>;
    fn parse_comparison(&self, value: Pair<Self::R>) -> ParserResult<Comparison>;
    fn parse_constraint(&self, value: Pair<Self::R>) -> ParserResult<Constraint>;
    fn parse_operator(&self, value: Pair<Self::R>) -> ParserResult<Operator>;
    fn parse_expr(&self, value: Pair<Self::R>) -> ParserResult<Expr>;
}
