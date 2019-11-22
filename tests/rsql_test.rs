use rsql::parser::rsql::*;
use rsql::parser::Parser;
use rsql::Comparison;
use rsql::Expr;
use rsql::Operator;

#[test]
fn test_simple() -> anyhow::Result<()> {
    let parser = RsqlParser::default();

    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("name", Comparison::EQUAL(), &["Kill Bill"])?,
        Expr::boxed_item("year", Comparison::GREATER_THAN(), &["2003"])?,
    );

    let code = r#"name=="Kill Bill";year=gt=2003"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    let code = r#"name=="Kill Bill" and year>2003"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_array() -> anyhow::Result<()> {
    let parser = RsqlParser::default();
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("director", Comparison::EQUAL(), &["Christopher Nolan"])?,
        Expr::boxed_item("actor", Comparison::EQUAL(), &["*Bale"])?,
    );

    let node = Expr::Node(
        Operator::And,
        Box::new(node),
        Expr::boxed_item("year", Comparison::GREATER_THAN_OR_EQUAL(), &["2000"])?,
    );

    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("genres", Comparison::IN(), &["sci-fi", "action"])?,
        Box::new(node),
    );

    let code =
        r#"genres=in=(sci-fi,action);(director=='Christopher Nolan',actor==*Bale);year=ge=2000"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    let code = r#"genres=in=(sci-fi,action) and (director=='Christopher Nolan' or actor==*Bale) and year>=2000"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_sub_fields() -> anyhow::Result<()> {
    let parser = RsqlParser::default();
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("year", Comparison::GREATER_THAN_OR_EQUAL(), &["2000"])?,
        Expr::boxed_item("year", Comparison::LESS_THAN(), &["2010"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("director.lastName", Comparison::EQUAL(), &["Nolan"])?,
        Box::new(node),
    );

    let code = r#"director.lastName==Nolan;year=ge=2000;year=lt=2010"#;
    assert_eq!(parser.parse_to_node(&code)?, node);
    let code = r#"director.lastName==Nolan and year>=2000 and year<2010"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    Ok(())
}

#[test]
fn test_double_array() -> anyhow::Result<()> {
    let parser = RsqlParser::default();
    let node = Expr::Node(
        Operator::Or,
        Expr::boxed_item("genres", Comparison::OUT(), &["romance", "animated", "horror"])?,
        Expr::boxed_item("director", Comparison::EQUAL(), &["Que*Tarantino"])?,
    );
    let node = Expr::Node(
        Operator::And,
        Expr::boxed_item("genres", Comparison::IN(), &["sci-fi", "action"])?,
        Box::new(node),
    );

    let code = r#"genres=in=(sci-fi,action) and genres=out=(romance,animated,horror) or director==Que*Tarantino"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    let code =
        r#"genres=in=(sci-fi,action);genres=out=(romance,animated,horror),director==Que*Tarantino"#;
    assert_eq!(parser.parse_to_node(&code)?, node);

    Ok(())
}
