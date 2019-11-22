use crate::Comparison;
use crate::Constraint;
use crate::Operator;
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
        selector: &str, comparison: Comparison, arguments: &[&str],
    ) -> ParserResult<Box<Expr>> {
        let res = Constraint::new(selector, comparison, arguments)?;
        Ok(Box::new(Expr::Item(res)))
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::comparison::Comparison;
    use crate::ast::constraint::Constraint;
    use crate::ast::expr::Expr;
    use crate::ast::Operator;
    use crate::ParserResult;

    #[test]
    fn test_node() -> ParserResult<()> {
        let const1 = Expr::Item(Constraint::new("select1", Comparison::EQUAL(), &["test1a"])?);
        let const2 = Expr::Item(Constraint::new("select2", Comparison::NOT_EQUAL(), &["test2a"])?);
        let const3 =
            Expr::Item(Constraint::new("select3", Comparison::GREATER_THAN(), &["test3a"])?);
        let const4 =
            Expr::Item(Constraint::new("select4", Comparison::IN(), &["test4a", "test4b"])?);

        let node1 = Expr::Node(Operator::And, Box::new(const1), Box::new(const3));
        let node2 = Expr::Node(Operator::Or, Box::new(const2), Box::new(const4));
        let _root = Expr::Node(Operator::And, Box::new(node1), Box::new(node2));

        Ok(())
    }
}
