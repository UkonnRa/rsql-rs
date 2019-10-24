use crate::ast::comparison::Comparison;
use crate::error::ParserError;
use crate::ParserResult;

#[derive(Debug, PartialEq, Eq)]
pub struct Constraint {
    pub selector: String,
    pub comparison: Comparison,
    pub arguments: Vec<String>,
}

impl Constraint {
    pub fn new(selector: &str, comparision: &Comparison, arguments: &[&str]) -> ParserResult<Self> {
        if comparision.multi_values {
            let expect_args = "> 1".to_string();
            if arguments.len() <= 1 {
                return Err(
                    ParserError::InvalidConstraintArgs(expect_args, arguments.len()).into(),
                );
            }
        } else {
            let expect_args = "== 1".to_string();
            if arguments.len() != 1 {
                return Err(
                    ParserError::InvalidConstraintArgs(expect_args, arguments.len()).into(),
                );
            }
        }

        Ok(Constraint {
            selector: selector.to_string(),
            comparison: comparision.clone(),
            arguments: arguments.iter().map(ToString::to_string).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::comparison;
    use crate::ast::comparison::Comparison;
    use crate::ast::constraint::Constraint;
    use crate::ParserResult;

    #[test]
    fn test_new() -> ParserResult<()> {
        let comp_eq: &Comparison = &comparison::EQUAL as &Comparison;
        let constraint = Constraint::new("name", comp_eq, &["new_name"])?;
        println!("{:?}", constraint);
        Ok(())
    }
}
