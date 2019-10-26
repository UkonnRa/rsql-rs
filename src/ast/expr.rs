use crate::ast::comparison::Comparison;
use crate::ast::constraint::Constraint;
use crate::ast::Operator;
use crate::ParserResult;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", content = "@data")]
pub enum Expr {
    Item(Constraint),
    Node(Operator, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn boxed_item(
        selector: &str, comparision: &Comparison, arguments: &[&str],
    ) -> ParserResult<Box<Expr>> {
        let res = Constraint::new(selector, comparision, arguments)?;
        Ok(Box::new(Expr::Item(res)))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::comparison;
    use crate::ast::comparison::Comparison;
    use crate::ast::constraint::Constraint;
    use crate::ast::expr::Expr;
    use crate::ast::Operator;
    use crate::ParserResult;

    #[test]
    fn test_node() -> ParserResult<()> {
        let const1 =
            Expr::Item(Constraint::new("select1", &comparison::EQUAL as &Comparison, &["test1a"])?);
        let const2 =
            Expr::Item(Constraint::new("select2", &comparison::NOT_EQUAL as &Comparison, &[
                "test2a",
            ])?);
        let const3 =
            Expr::Item(Constraint::new("select3", &comparison::GREATER_THAN as &Comparison, &[
                "test3a",
            ])?);
        let const4 = Expr::Item(Constraint::new("select4", &comparison::IN as &Comparison, &[
            "test4a", "test4b",
        ])?);

        let node1 = Expr::Node(Operator::And, Box::new(const1), Box::new(const3));
        let node2 = Expr::Node(Operator::Or, Box::new(const2), Box::new(const4));
        let root = Expr::Node(Operator::And, Box::new(node1), Box::new(node2));

        println!("root: {:?}", root);

        Ok(())
    }
}
