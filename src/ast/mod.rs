pub mod comparison;
pub mod constraint;
pub mod expr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    And,
    Or,
}
