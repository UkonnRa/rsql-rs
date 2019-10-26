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
        Operator::And,
        Expr::boxed_item("updated", &LESS_THAN as &Comparison, &["-P1D"])?,
        Expr::boxed_item("title", &EQUAL as &Comparison, &["*b%20r"])?,
    );
    let node = Expr::Node(
        Operator::Or,
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
