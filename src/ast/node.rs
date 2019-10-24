use crate::ast::constraint::Constraint;
use crate::ast::Operator;
use std::fmt::Debug;

pub trait Node {}

#[derive(Debug, PartialEq, Eq)]
pub struct Leaf(Constraint);

impl Node for Leaf {}

#[derive(Debug, PartialEq, Eq)]
pub struct Branch<T>
where
    T: Node,
{
    pub operator: Operator,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Branch<T>
where
    T: Node,
{
    fn new(operator: Operator, left: Option<T>, right: Option<T>) -> Self {
        Branch {
            operator,
            left,
            right,
        }
    }
}

impl<T: Node> Node for Branch<T> {}

#[cfg(test)]
mod tests {
    use crate::ast::comparison;
    use crate::ast::comparison::Comparison;
    use crate::ast::constraint::Constraint;
    use crate::ast::node::{Branch, Leaf};
    use crate::ast::Operator;
    use crate::ParserResult;

    #[test]
    fn test_node() -> ParserResult<()> {
        let const1 = Leaf(Constraint::new(
            "select1",
            &comparison::EQUAL as &Comparison,
            &["test1a"],
        )?);
        let const2 = Leaf(Constraint::new(
            "select2",
            &comparison::NOT_EQUAL as &Comparison,
            &["test2a"],
        )?);
        let const3 = Leaf(Constraint::new(
            "select3",
            &comparison::GREATER_THAN as &Comparison,
            &["test3a"],
        )?);
        let const4 = Leaf(Constraint::new(
            "select4",
            &comparison::IN as &Comparison,
            &["test4a", "test4b"],
        )?);

        let branch1 = Branch::new(Operator::And, Some(const1), Some(const3));
        let branch2 = Branch::new(Operator::And, Some(const2), Some(const4));

        let root = Branch::new(Operator::Or, Some(branch1), Some(branch2));

        println!("root: {:?}", root);

        Ok(())
    }
}
