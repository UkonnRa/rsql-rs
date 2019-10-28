use rsql_rs::ast::comparison::Comparison;
use rsql_rs::ast::comparison::*;
use rsql_rs::ast::expr::Expr;
use rsql_rs::ast::Operator;
use rsql_rs::parser::rsql::*;
use rsql_rs::parser::Parser;

#[test]
fn test_simple() -> anyhow::Result<()> {
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("name", &EQUAL as &Comparison, &["Kill Bill"])?,
        Expr::boxed_item("year", &GREATER_THAN as &Comparison, &["2003"])?,
    );

    let code = r#"name=="Kill Bill";year=gt=2003"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    let code = r#"name=="Kill Bill" and year>2003"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_array() -> anyhow::Result<()> {
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("director", &EQUAL as &Comparison, &["Christopher Nolan"])?,
        Expr::boxed_item("actor", &EQUAL as &Comparison, &["*Bale"])?,
    );

    let node = Expr::Node(
        Operator::And,
        Box::new(node),
        Expr::boxed_item("year", &GREATER_THAN_OR_EQUAL as &Comparison, &["2000"])?,
    );

    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("genres", &IN as &Comparison, &["sci-fi", "action"])?,
        Box::new(node),
    );

    let code =
        r#"genres=in=(sci-fi,action);(director=='Christopher Nolan',actor==*Bale);year=ge=2000"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    let code = r#"genres=in=(sci-fi,action) and (director=='Christopher Nolan' or actor==*Bale) and year>=2000"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_sub_fields() -> anyhow::Result<()> {
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("year", &GREATER_THAN_OR_EQUAL as &Comparison, &["2000"])?,
        Expr::boxed_item("year", &LESS_THAN as &Comparison, &["2010"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("director.lastName", &EQUAL as &Comparison, &["Nolan"])?,
        Box::new(node),
    );

    let code = r#"director.lastName==Nolan;year=ge=2000;year=lt=2010"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);
    let code = r#"director.lastName==Nolan and year>=2000 and year<2010"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_double_array() -> anyhow::Result<()> {
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("genres", &OUT as &Comparison, &["romance", "animated", "horror"])?,
        Expr::boxed_item("director", &EQUAL as &Comparison, &["Que*Tarantino"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("genres", &IN as &Comparison, &["sci-fi", "action"])?,
        Box::new(node),
    );

    let code = r#"genres=in=(sci-fi,action) and genres=out=(romance,animated,horror) or director==Que*Tarantino"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    let code =
        r#"genres=in=(sci-fi,action);genres=out=(romance,animated,horror),director==Que*Tarantino"#;
    assert_eq!(RsqlParser::parse_to_node(&code)?, node);

    Ok(())
}
