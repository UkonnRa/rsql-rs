pub mod comparison;
pub mod constraint;
pub mod expr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Operator {
    And,
    Or,
}
