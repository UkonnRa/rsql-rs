use crate::ast::comparison::Comparison;
use crate::ast::constraint::{Arguments, Constraint};
use crate::ast::expr::Expr;
use crate::ast::{comparison, Operator};
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

impl Parser for RsqlParser {
    type R = Rule;

    fn parse_to_node(code: &str) -> ParserResult<Expr> {
        let res = Self::parse(Rule::expression, &code)?.next().unwrap();
        let res: Expr = res.try_into()?;
        Ok(res)
    }

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison> {
        &DEFAULT_COMPS_MAP as &'static HashMap<&'static str, &'static Comparison>
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Arguments {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        println!("  value: {:?}", value);

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

impl<'i> TryFrom<Pair<'i, Rule>> for Comparison {
    type Error = ParserError;

    fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::comparison => {
                let comp_name = value.as_str();
                if let Some(&comp) = RsqlParser::default_comparisons().get(comp_name) {
                    Ok(comp.clone())
                } else {
                    Err(ParserError::InvalidComparison(comp_name.to_string()))
                }
            },
            _ => ParserError::invalid_pair_rule()?,
        }
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Constraint {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut selector_opt: Option<String> = None;
        let mut comparison_opt: Option<Comparison> = None;
        let mut arguments_opt: Option<Arguments> = None;

        match value.as_rule() {
            Rule::constraint => {
                for item in value.into_inner() {
                    match item.as_rule() {
                        Rule::selector => selector_opt = Some(item.as_str().to_string()),
                        Rule::comparison => comparison_opt = item.try_into().ok(),
                        Rule::argument => {
                            println!("arguments: {:?}", item);
                            arguments_opt = item.try_into().ok()
                        },
                        _ => {},
                    }
                }
                let selector = if let Some(selector) = selector_opt {
                    selector
                } else {
                    return Err(ParserError::LackOfField {
                        ty: "Constraint".to_string(),
                        field: "selector".to_string(),
                    });
                };
                let comparison = if let Some(comparison) = comparison_opt {
                    comparison
                } else {
                    return Err(ParserError::LackOfField {
                        ty: "Constraint".to_string(),
                        field: "comparison".to_string(),
                    });
                };
                let arguments = if let Some(arguments) = arguments_opt {
                    arguments
                } else {
                    return Err(ParserError::LackOfField {
                        ty: "Constraint".to_string(),
                        field: "arguments".to_string(),
                    });
                };

                Ok(Constraint { selector, comparison, arguments })
            },
            _ => ParserError::invalid_pair_rule()?,
        }
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Operator {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::operator => match value.into_inner().next() {
                Some(pair) if pair.as_rule() == Rule::and_op => Ok(Operator::And),
                Some(pair) if pair.as_rule() == Rule::or_op => Ok(Operator::Or),
                _ => ParserError::invalid_pair_rule()?,
            },
            _ => ParserError::invalid_pair_rule()?,
        }
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Expr {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut op_vec: Vec<Operator> = vec![];
        let mut expr_vec: Vec<Expr> = vec![];

        let mut parse_op = |pair: Pair<'i, Rule>| -> ParserResult<()> {
            match pair.as_rule() {
                Rule::operator if vec![";", "and"].contains(&pair.as_str()) => {
                    op_vec.push(Operator::And)
                },
                Rule::operator if vec![",", "or"].contains(&pair.as_str()) => {
                    op_vec.push(Operator::Or)
                },
                _ => ParserError::invalid_pair_rule()?,
            }
            Ok(())
        };

        match value.as_rule() {
            Rule::expression => {
                for expr_item in value.into_inner() {
                    match expr_item.as_rule() {
                        Rule::constraint => expr_vec.push(Expr::Item(expr_item.try_into()?)),
                        Rule::group => expr_vec.push(Expr::try_from(expr_item)?),
                        Rule::operator => parse_op(expr_item)?,
                        _ => ParserError::invalid_pair_rule()?,
                    }
                }
            },
            Rule::group => {
                for group_item in value.into_inner() {
                    match group_item.as_rule() {
                        Rule::expression => expr_vec.push(Expr::try_from(group_item)?),
                        Rule::operator => parse_op(group_item)?,
                        _ => ParserError::invalid_pair_rule()?,
                    }
                }
            },
            _ => ParserError::invalid_pair_rule()?,
        }

        while let Some(top_op) = op_vec.pop() {
            if expr_vec.len() < 2 {
                ParserError::invalid_pair_rule()?
            } else {
                let right = expr_vec.pop().unwrap();
                let left = expr_vec.pop().unwrap();
                expr_vec.push(Expr::Node(top_op, Box::new(left), Box::new(right)));
            }
        }

        if op_vec.is_empty() && expr_vec.len() == 1 {
            Ok(expr_vec.pop().unwrap())
        } else {
            ParserError::invalid_pair_rule()?
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
