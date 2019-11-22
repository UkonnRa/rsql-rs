use crate::error::ParserError;
use crate::Comparison;
use crate::ParserResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arguments(pub Vec<String>);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constraint {
    pub selector: String,
    pub comparison: Comparison,
    pub arguments: Arguments,
}

impl Constraint {
    pub fn new(selector: &str, comparison: Comparison, arguments: &[&str]) -> ParserResult<Self> {
        if comparison.multi_values {
            let expect_args = "> 1".to_string();
            if arguments.len() <= 1 {
                return Err(ParserError::InvalidConstraintArgs(expect_args, arguments.len()));
            }
        } else {
            let expect_args = "== 1".to_string();
            if arguments.len() != 1 {
                return Err(ParserError::InvalidConstraintArgs(expect_args, arguments.len()));
            }
        }

        Ok(Constraint {
            selector: selector.to_string(),
            comparison,
            arguments: Arguments(arguments.iter().map(ToString::to_string).collect()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::comparison::Comparison;
    use crate::ast::constraint::Constraint;
    use crate::ParserResult;

    #[test]
    fn test_new() -> ParserResult<()> {
        let constraint = Constraint::new("name", Comparison::EQUAL(), &["new_name"])?;
        println!("{:?}", constraint);
        Ok(())
    }
}
