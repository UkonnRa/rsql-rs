use crate::error::ParserError;
use crate::Comparison;
use crate::ParserResult;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Arguments(pub Vec<String>);

static RESERVED_CHARS: &[char] = &['"', '\'', '(', ')', ';', ',', '=', '!', '~', '<', '>', ' '];

fn add_quote(arg: &str) -> String {
    if arg.find(|c| RESERVED_CHARS.contains(&c)).is_some() {
        let mut escaped = false;
        let mut should_double = false;

        for c in arg.chars() {
            if c == '\\' {
                escaped = true;
            } else if c == '\'' && !escaped {
                should_double = true;
                break;
            } else if c == '"' && !escaped {
                break;
            } else {
                escaped = false;
            }
        }

        if should_double {
            format!(r#""{}""#, arg)
        } else {
            format!("'{}'", arg)
        }
    } else {
        arg.to_string()
    }
}

impl ToString for Arguments {
    fn to_string(&self) -> String {
        if self.0.len() > 1 {
            format!("({})", self.0.iter().map(|s| add_quote(s.as_str())).join(","))
        } else {
            add_quote(&self.0.first().cloned().unwrap_or_default())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub selector: String,
    pub comparison: Comparison,
    pub arguments: Arguments,
}

impl ToString for Constraint {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.selector, self.comparison.to_string(), self.arguments.to_string())
    }
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
    use crate::{Arguments, ParserResult};

    #[test]
    fn test_new() -> ParserResult<()> {
        let constraint = Constraint::new("name", Comparison::EQUAL(), &["new_name"])?;
        assert_eq!(constraint.to_string(), "name==new_name");
        Ok(())
    }

    #[test]
    fn arguments_to_string() {
        let args = Arguments(
            vec!["String", "Hello World!", r#""double quoted""#, "it's really", "only\"test"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        );
        assert_eq!(
            args.to_string(),
            r#"(String,'Hello World!','"double quoted"',"it's really",'only"test')"#
        )
    }
}
