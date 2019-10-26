use crate::error::ParserError;
use crate::ParserResult;
use regex::RegexSet;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref COMP_OP_RE: RegexSet = RegexSet::new(&[r"^=[a-zA-Z]*=$", r"^[<>]=?$"]).unwrap();
    pub static ref EQUAL: Comparison = Comparison::new(&["=="], false).unwrap();
    pub static ref NOT_EQUAL: Comparison = Comparison::new(&["!="], false).unwrap();
    pub static ref GREATER_THAN: Comparison = Comparison::new(&[">", "=gt="], false).unwrap();
    pub static ref GREATER_THAN_OR_EQUAL: Comparison =
        Comparison::new(&[">=", "=ge="], false).unwrap();
    pub static ref LESS_THAN: Comparison = Comparison::new(&["<", "=lt="], false).unwrap();
    pub static ref LESS_THAN_OR_EQUAL: Comparison =
        Comparison::new(&["<=", "=le="], false).unwrap();
    pub static ref IN: Comparison = Comparison::new(&["=in="], true).unwrap();
    pub static ref OUT: Comparison = Comparison::new(&["=out="], true).unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Comparison {
    pub symbols: Vec<String>,
    pub multi_values: bool,
}

impl Comparison {
    pub fn new(symbols: &[&str], multi_values: bool) -> ParserResult<Comparison> {
        let symbols = symbols
            .iter()
            .map(|&sym| Self::is_valid_symbol(sym))
            .collect::<ParserResult<Vec<String>>>()?;
        Ok(Comparison {
            symbols,
            multi_values,
        })
    }

    fn is_valid_symbol(symbol: &str) -> ParserResult<String> {
        if COMP_OP_RE.is_match(symbol) || symbol == "!=" {
            Ok(symbol.to_string())
        } else {
            Err(ParserError::InvalidComparison(symbol.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::comparison::*;

    #[test]
    fn test_valid() -> anyhow::Result<()> {
        let symbols = vec!["==", "=eq="];
        Comparison::new(&symbols, false)?;

        Ok(())
    }

    #[test]
    fn test_invalid() -> anyhow::Result<()> {
        assert!(Comparison::new(&["~="], false).is_err());
        assert!(Comparison::new(&["><="], false).is_err());
        assert!(Comparison::new(&["test="], false).is_err());
        Ok(())
    }
}
