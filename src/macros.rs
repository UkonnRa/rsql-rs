macro_rules! default_comparisons {
    (RSQL, $symbol:ident) => {{
        match $symbol {
            "==" => Some(crate::Comparison::EQUAL()),
            "!=" => Some(crate::Comparison::NOT_EQUAL()),
            ">" | "=gt=" => Some(crate::Comparison::GREATER_THAN()),
            ">=" | "=ge=" => Some(crate::Comparison::GREATER_THAN_OR_EQUAL()),
            "<" | "=lt=" => Some(crate::Comparison::LESS_THAN()),
            "<=" | "=le=" => Some(crate::Comparison::LESS_THAN_OR_EQUAL()),
            "=in=" => Some(crate::Comparison::IN()),
            "=out=" => Some(crate::Comparison::OUT()),
            _ => None,
        }
    }};
    (FIQL, $symbol:ident) => {{
        match $symbol {
            "==" => Some(crate::Comparison::EQUAL()),
            "!=" => Some(crate::Comparison::NOT_EQUAL()),
            ">" | "=gt=" => Some(crate::Comparison::GREATER_THAN()),
            ">=" | "=ge=" => Some(crate::Comparison::GREATER_THAN_OR_EQUAL()),
            "<" | "=lt=" => Some(crate::Comparison::LESS_THAN()),
            "<=" | "=le=" => Some(crate::Comparison::LESS_THAN_OR_EQUAL()),
            _ => None,
        }
    }};
}

#[macro_export]
macro_rules! gen_basic_parser {
    ($ty:ident) => {
        fn parse_to_node(&self, code: &str) -> crate::ParserResult<crate::Expr> {
            let res = Self::parse(Rule::expression, &code)?.next().unwrap();
            self.parse_expr(res)
        }

        fn get_inner_mut(&mut self) -> &mut std::collections::HashMap<String, crate::Comparison> {
            &mut self.0
        }

        fn get_inner(&self) -> &std::collections::HashMap<String, crate::Comparison> {
            &self.0
        }

        fn register_comparison(&mut self, comparison: &crate::Comparison) {
            let map = self.get_inner_mut();
            for k in &comparison.symbols {
                map.insert(k.clone(), comparison.clone());
            }
        }

        fn remove_comparison_by_symbol(&mut self, symbol: &str) {
            self.get_inner_mut().remove(symbol);
        }

        fn get_comparison(&self, symbol: &str) -> Option<crate::Comparison> {
            let additional = default_comparisons!($ty, symbol);
           self.get_inner().get(symbol).cloned().or(additional)
        }

        fn parse_comparison(&self, value: pest::iterators::Pair<Rule>) -> crate::ParserResult<crate::Comparison> {
            match value.as_rule() {
                Rule::comparison => {
                    let comp_name = value.as_str();
                    if let Some(comp) = self.get_comparison(comp_name) {
                        Ok(comp)
                    } else {
                        Err(crate::error::ParserError::InvalidComparison(comp_name.to_string()))
                    }
                },
                _ => crate::error::ParserError::invalid_pair_rule()?,
            }
        }

        fn parse_constraint(&self, value: pest::iterators::Pair<Rule>) -> crate::ParserResult<crate::Constraint> {
            let mut selector_opt: std::option::Option<String> = None;
            let mut comparison_opt: std::option::Option<crate::Comparison> = None;
            let mut arguments_opt: std::option::Option<crate::Arguments> = None;

            match value.as_rule() {
                Rule::constraint => {
                    for item in value.into_inner() {
                        match item.as_rule() {
                            Rule::selector => selector_opt = Some(item.as_str().to_string()),
                            Rule::comparison => comparison_opt = Some(self.parse_comparison(item)?),
                            Rule::argument => arguments_opt = Some(item.try_into()?),
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

                    Ok(crate::Constraint { selector, comparison, arguments })
                },
                _ => crate::error::ParserError::invalid_pair_rule()?,
            }
        }

        fn parse_operator(&self, value: pest::iterators::Pair<Self::R>) -> crate::ParserResult<crate::Operator> {
            match value.as_rule() {
                Rule::operator => match value.into_inner().next() {
                    Some(pair) if pair.as_rule() == Rule::and_op => {
                        Ok(crate::ast::Operator::And)
                    }
                    Some(pair) if pair.as_rule() == Rule::or_op => Ok(crate::Operator::Or),
                    _ => crate::error::ParserError::invalid_pair_rule()?,
                },
                _ => crate::error::ParserError::invalid_pair_rule()?,
            }
        }

        fn parse_expr(&self, value: pest::iterators::Pair<Self::R>) -> crate::ParserResult<crate::Expr> {
            let mut op_vec: std::vec::Vec<crate::Operator> = vec![];
            let mut expr_vec: std::vec::Vec<crate::Expr> = vec![];

            let mut parse_op =
                |pair: pest::iterators::Pair<Rule>| -> crate::ParserResult<()> {
                    match pair.as_rule() {
                        Rule::operator if vec![";", "and"].contains(&pair.as_str()) => {
                            op_vec.push(crate::Operator::And)
                        }
                        Rule::operator if vec![",", "or"].contains(&pair.as_str()) => {
                            op_vec.push(crate::Operator::Or)
                        }
                        _ => crate::error::ParserError::invalid_pair_rule()?,
                    }
                    Ok(())
                };

            match value.as_rule() {
                Rule::expression => {
                    for expr_item in value.into_inner() {
                        match expr_item.as_rule() {
                            Rule::constraint => {
                                expr_vec.push(crate::Expr::Item(self.parse_constraint(expr_item)?))
                            }
                            Rule::group => expr_vec.push(self.parse_expr(expr_item)?),
                            Rule::operator => parse_op(expr_item)?,
                            _ => crate::error::ParserError::invalid_pair_rule()?,
                        }
                    }
                }
                Rule::group => {
                    for group_item in value.into_inner() {
                        match group_item.as_rule() {
                            Rule::expression => expr_vec.push(self.parse_expr(group_item)?),
                            Rule::operator => parse_op(group_item)?,
                            _ => crate::error::ParserError::invalid_pair_rule()?,
                        }
                    }
                }
                _ => crate::error::ParserError::invalid_pair_rule()?,
            }

            while let Some(top_op) = op_vec.pop() {
                if expr_vec.len() < 2 {
                    crate::error::ParserError::invalid_pair_rule()?
                } else {
                    let right = expr_vec.pop().unwrap();
                    let left = expr_vec.pop().unwrap();
                    expr_vec.push(crate::ast::expr::Expr::Node(
                        top_op,
                        Box::new(left),
                        Box::new(right),
                    ));
                }
            }

            if op_vec.is_empty() && expr_vec.len() == 1 {
                Ok(expr_vec.pop().unwrap())
            } else {
                crate::error::ParserError::invalid_pair_rule()?
            }
        }
    };
}

#[macro_export]
macro_rules! gen_parser {
    ($class_name:ident) => {
        impl From<&[crate::Comparison]> for $class_name {
            fn from(comparisons: &[crate::Comparison]) -> Self {
                Self(
                    comparisons
                        .iter()
                        .flat_map(|c| c.symbols.iter().map(move |sym| (sym.clone(), c.clone())))
                        .collect(),
                )
            }
        }
        impl From<Vec<crate::Comparison>> for $class_name {
            fn from(comparisons: Vec<crate::Comparison>) -> Self {
                comparisons.as_slice().into()
            }
        }
    };
}
