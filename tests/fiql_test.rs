use rsql::parser::fiql::*;
use rsql::parser::rsql::RsqlParser;
use rsql::parser::Parser;
use rsql::Comparison;
use rsql::Expr;
use rsql::Operator;

#[test]
fn test_simple() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();

    let code = "title==foo*;(updated=lt=-P1D,title==*b%20r)";
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("updated", Comparison::LESS_THAN(), &["-P1D"])?,
        Expr::boxed_item("title", Comparison::EQUAL(), &["*b%20r"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("title", Comparison::EQUAL(), &["foo*"])?,
        Box::new(node),
    );

    assert_eq!(fiql_parser.parse_to_node(&code)?, node);
    assert_eq!(rsql_parser.parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_date() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();

    let code = "updated==2003-12-13T18:30:02Z";
    let node = Expr::boxed_item("updated", Comparison::EQUAL(), &["2003-12-13T18:30:02Z"])?;
    assert_eq!(Box::new(fiql_parser.parse_to_node(&code)?), node);
    assert_eq!(Box::new(rsql_parser.parse_to_node(&code)?), node);
    Ok(())
}

#[test]
fn test_number() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();
    let code = "x:foo=gt=500";
    let node = Expr::boxed_item("x:foo", Comparison::GREATER_THAN(), &["500"])?;
    assert_eq!(Box::new(fiql_parser.parse_to_node(&code)?), node);
    assert_eq!(Box::new(rsql_parser.parse_to_node(&code)?), node);
    Ok(())
}

#[test]
fn test_parentheses_lacking() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();
    let code = "(lack==of_paren";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    assert!(rsql_parser.parse_to_node(&code).is_err());
    Ok(())
}

#[test]
fn test_invalid_selector() -> anyhow::Result<()> {
    let _rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();
    let code = "$invalid==argument";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    let code = "!invalid==argument";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    let code = "*invalid==argument";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    let code = "+invalid==argument";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    let code = "'invalid==argument";
    assert!(fiql_parser.parse_to_node(&code).is_err());

    Ok(())
}

#[test]
fn test_invalid_comparison() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();
    let code = "key=!=value";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    assert!(rsql_parser.parse_to_node(&code).is_err());
    let code = "key=~=value";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    assert!(rsql_parser.parse_to_node(&code).is_err());
    let code = "key=notfound=value";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    assert!(rsql_parser.parse_to_node(&code).is_err());
    let code = "key<>=value";
    assert!(fiql_parser.parse_to_node(&code).is_err());
    assert!(rsql_parser.parse_to_node(&code).is_err());

    Ok(())
}

#[test]
fn test_complex() -> anyhow::Result<()> {
    let rsql_parser = RsqlParser::default();
    let fiql_parser = FiqlParser::default();
    let code = "updated==2003-12-13T18:30:02Z;(director==Christopher%20Nolan,(actor==*Bale;\
                year=ge=1.234),content==*just%20the%20start*)";
    let actor_year = Expr::Node(
        Operator::And,
        Expr::boxed_item("actor", Comparison::EQUAL(), &["*Bale"])?,
        Expr::boxed_item("year", Comparison::GREATER_THAN_OR_EQUAL(), &["1.234"])?,
    );
    let res = Expr::Node(
        Operator::Or,
        Box::new(actor_year),
        Expr::boxed_item("content", Comparison::EQUAL(), &["*just%20the%20start*"])?,
    );
    let res = Expr::Node(
        Operator::Or,
        Expr::boxed_item("director", Comparison::EQUAL(), &["Christopher%20Nolan"])?,
        Box::new(res),
    );
    let res = Expr::Node(
        Operator::And,
        Expr::boxed_item("updated", Comparison::EQUAL(), &["2003-12-13T18:30:02Z"])?,
        Box::new(res),
    );

    assert_eq!(fiql_parser.parse_to_node(&code)?, res);
    assert_eq!(rsql_parser.parse_to_node(&code)?, res);

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
