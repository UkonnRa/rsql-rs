#[macro_export]
macro_rules! gen_parser_to_node {
    () => {
        fn parse_to_node(code: &str) -> ParserResult<Expr> {
            let res = Self::parse(Rule::expression, &code)?.next().unwrap();
            let res: Expr = res.try_into()?;
            Ok(res)
        }
    };
}

#[macro_export]
macro_rules! gen_parser {
    ($class_name:ident) => {
        impl<'i> TryFrom<Pair<'i, Rule>> for Comparison {
            type Error = ParserError;

            fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
                match value.as_rule() {
                    Rule::comparison => {
                        let comp_name = value.as_str();
                        if let Some(&comp) = $class_name::default_comparisons().get(comp_name) {
                            Ok(comp.clone())
                        } else {
                            Err(ParserError::InvalidComparison(comp_name.to_string()))
                        }
                    },
                    _ => ParserError::invalid_pair_rule()?,
                }
            }
        }

        impl<'i> TryFrom<Pair<'i, Rule>> for crate::ast::constraint::Constraint {
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
                                Rule::argument => arguments_opt = item.try_into().ok(),
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

                        Ok(crate::ast::constraint::Constraint { selector, comparison, arguments })
                    },
                    _ => ParserError::invalid_pair_rule()?,
                }
            }
        }

        impl<'i> TryFrom<Pair<'i, Rule>> for crate::ast::Operator {
            type Error = ParserError;

            fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
                match value.as_rule() {
                    Rule::operator => match value.into_inner().next() {
                        Some(pair) if pair.as_rule() == Rule::and_op => {
                            Ok(crate::ast::Operator::And)
                        },
                        Some(pair) if pair.as_rule() == Rule::or_op => Ok(crate::ast::Operator::Or),
                        _ => ParserError::invalid_pair_rule()?,
                    },
                    _ => ParserError::invalid_pair_rule()?,
                }
            }
        }

        impl<'i> TryFrom<Pair<'i, Rule>> for Expr {
            type Error = ParserError;

            fn try_from(value: Pair<'i, Rule>) -> Result<Self, Self::Error> {
                let mut op_vec: Vec<crate::ast::Operator> = vec![];
                let mut expr_vec: Vec<Expr> = vec![];

                let mut parse_op = |pair: Pair<'i, Rule>| -> ParserResult<()> {
                    match pair.as_rule() {
                        Rule::operator if vec![";", "and"].contains(&pair.as_str()) => {
                            op_vec.push(crate::ast::Operator::And)
                        },
                        Rule::operator if vec![",", "or"].contains(&pair.as_str()) => {
                            op_vec.push(crate::ast::Operator::Or)
                        },
                        _ => ParserError::invalid_pair_rule()?,
                    }
                    Ok(())
                };

                match value.as_rule() {
                    Rule::expression => {
                        for expr_item in value.into_inner() {
                            match expr_item.as_rule() {
                                Rule::constraint => {
                                    expr_vec.push(Expr::Item(expr_item.try_into()?))
                                },
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
    };
}
