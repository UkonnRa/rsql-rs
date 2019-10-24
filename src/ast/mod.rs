pub mod comparison;
pub mod constraint;
pub mod node;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
}
