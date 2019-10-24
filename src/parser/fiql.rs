use crate::ast::comparison;
use crate::ast::comparison::Comparison;
use crate::ast::constraint::Constraint;
use crate::ast::node::{Branch, Node};
use crate::error::ParserError;
use crate::parser::Parser;
use crate::ParserResult;
use pest::iterators::Pair;
use std::backtrace::Backtrace;
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

impl Parser for FiqlParser {
    type R = Rule;
    fn parse_to_node(code: &str) -> ParserResult<Box<dyn Node>> {
        unimplemented!()
    }

    fn default_comparisons() -> &'static HashMap<&'static str, &'static Comparison> {
        &DEFAULT_COMPS_MAP as &'static HashMap<&'static str, &'static Comparison>
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Comparison {
    type Error = ParserError;

    fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::comparison => {
                let comp_name = value.as_str();
                if let Some(&comp) = FiqlParser::default_comparisons().get(comp_name) {
                    Ok(comp.clone())
                } else {
                    Err(ParserError::InvalidComparison(comp_name.to_string()))
                }
            }
            rule => Err(ParserError::InvalidPairRule(Backtrace::capture())),
        }
    }
}

impl<'i> TryFrom<Pair<'i, Rule>> for Constraint {
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut selector_opt: Option<String> = None;
        let mut comparison_opt: Option<Comparison> = None;
        let mut arguments_opt: Option<String> = None;

        match value.as_rule() {
            Rule::constraint => {
                for item in value.into_inner() {
                    match item.as_rule() {
                        Rule::selector => selector_opt = Some(item.as_str().to_string()),
                        Rule::comparison => comparison_opt = item.try_into().ok(),
                        Rule::argument => arguments_opt = Some(item.as_str().to_string()),
                        _ => {}
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
                    vec![arguments]
                } else {
                    return Err(ParserError::LackOfField {
                        ty: "Constraint".to_string(),
                        field: "arguments".to_string(),
                    });
                };

                Ok(Constraint {
                    selector,
                    comparison,
                    arguments,
                })
            }
            rule => Err(ParserError::InvalidPairRule(Backtrace::capture())),
        }
    }
}

impl<'i, T> TryFrom<Pair<'i, Rule>> for Branch<T>
where
    T: Node,
{
    type Error = ParserError;

    fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        for item in pairs.into_inner() {
            match item.as_rule() {
                Rule::group => {
                    info!("in an group:");
                    for part in item.into_inner() {
                        info!("    part: {:?}", part);
                        match Constraint::try_from(part) {
                            Ok(res) => info!("res: {:?}", res),
                            Err(err) => error!("err: {}", err),
                        }
                    }
                }
                Rule::operator => {
                    for part in item.into_inner() {
                        match part.as_rule() {
                            Rule::and_op => info!("  AND!"),
                            Rule::or_op => info!("  OR!"),
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::constraint::Constraint;
    use crate::ast::node::Node;
    use crate::error::ParserError;
    use crate::parser::fiql::*;
    use crate::parser::Parser;
    use crate::{ParserResult, QueryType};
    use log::*;
    use pest::Parser as PestParser;
    use std::backtrace::Backtrace;
    use std::convert::TryFrom;

    #[test]
    fn default_fiql_map_test() -> ParserResult<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let _ = FiqlParser::default_comparisons();
        Ok(())
    }

    #[test]
    fn fiql_parser_test() -> ParserResult<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut stack: Vec<Box<dyn Node>> = vec![];
        let code = " (updated == 2003-12-13T18:30:02Z);(director==Christopher Nolan,actor==*Bale);year=ge=1.234,(content==*just%20the%20start*)";
        let pairs = FiqlParser::parse(Rule::expression, code)?
            .next()
            .ok_or_else(|| ParserError::InvalidQuery(QueryType::Fiql, Backtrace::capture()))?;

        Ok(())
    }
}
