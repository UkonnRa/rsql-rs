use rsql_rs::ast::comparison::Comparison;
use rsql_rs::ast::comparison::*;
use rsql_rs::ast::expr::Expr;
use rsql_rs::ast::Operator;
use rsql_rs::parser::fiql::*;
use rsql_rs::parser::Parser;

#[test]
fn test_simple() -> anyhow::Result<()> {
    let code = "title==foo*;(updated=lt=-P1D,title==*b%20r)";
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("updated", &LESS_THAN as &Comparison, &["-P1D"])?,
        Expr::boxed_item("title", &EQUAL as &Comparison, &["*b%20r"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("title", &EQUAL as &Comparison, &["foo*"])?,
        Box::new(node),
    );

    assert_eq!(FiqlParser::parse_to_node(&code)?, node);
    Ok(())
}

#[test]
fn test_date() -> anyhow::Result<()> {
    let code = "updated==2003-12-13T18:30:02Z";
    let node = Expr::boxed_item("updated", &EQUAL as &Comparison, &["2003-12-13T18:30:02Z"])?;
    assert_eq!(Box::new(FiqlParser::parse_to_node(&code)?), node);
    Ok(())
}

#[test]
fn test_number() -> anyhow::Result<()> {
    let code = "x:foo=gt=500";
    let node = Expr::boxed_item("x:foo", &GREATER_THAN as &Comparison, &["500"])?;
    assert_eq!(Box::new(FiqlParser::parse_to_node(&code)?), node);
    Ok(())
}

#[test]
fn test_parentheses_lacking() -> anyhow::Result<()> {
    let code = "(lack==of_paren";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    Ok(())
}

#[test]
fn test_invalid_selector() -> anyhow::Result<()> {
    let code = "$invalid==argument";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "!invalid==argument";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "*invalid==argument";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "+invalid==argument";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "'invalid==argument";
    assert!(FiqlParser::parse_to_node(&code).is_err());

    Ok(())
}

#[test]
fn test_invalid_comparison() -> anyhow::Result<()> {
    let code = "key=!=value";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "key=~=value";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "key=notfound=value";
    assert!(FiqlParser::parse_to_node(&code).is_err());
    let code = "key<>=value";
    assert!(FiqlParser::parse_to_node(&code).is_err());

    Ok(())
}

#[test]
fn test_complex() -> anyhow::Result<()> {
    let code = "updated == 2003-12-13T18:30:02Z ; ( director == Christopher%20Nolan,  (actor== \
                *Bale ; year =ge= 1.234 ) , content==*just%20the%20start*)";
    let actor_year = Expr::Node(
        Operator::And,
        Expr::boxed_item("actor", &EQUAL as &Comparison, &["*Bale"])?,
        Expr::boxed_item("year", &GREATER_THAN_OR_EQUAL as &Comparison, &["1.234"])?,
    );
    let res = Expr::Node(
        Operator::Or,
        Box::new(actor_year),
        Expr::boxed_item("content", &EQUAL as &Comparison, &["*just%20the%20start*"])?,
    );
    let res = Expr::Node(
        Operator::Or,
        Expr::boxed_item("director", &EQUAL as &Comparison, &["Christopher%20Nolan"])?,
        Box::new(res),
    );
    let res = Expr::Node(
        Operator::And,
        Expr::boxed_item("updated", &EQUAL as &Comparison, &["2003-12-13T18:30:02Z"])?,
        Box::new(res),
    );

    assert_eq!(FiqlParser::parse_to_node(&code)?, res);

    Ok(())
}
