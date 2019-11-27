use crate::error::ParserError;
use crate::ParserResult;
use regex::RegexSet;
use serde::{Deserialize, Serialize};

macro_rules! default_comparisons {
    ($name:ident, $multi:expr, $($symbol:expr),+) => {
        #[allow(non_snake_case)]
        pub fn $name() -> Comparison {
            Comparison::new(&[$($symbol),+], $multi).unwrap()
        }
    };
    ( $($name:ident, $multi:expr, [$($symbol:expr),+];)+ ) => {
        impl Comparison {
            $(
                default_comparisons!($name, $multi, $($symbol),+);
            )+
        }
    }
}

lazy_static! {
    static ref COMP_OP_RE: RegexSet = RegexSet::new(&[r"^=[a-zA-Z]*=$", r"^[<>]=?$"]).unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Comparison {
    pub(crate) symbols: Vec<String>,
    pub(crate) multi_values: bool,
}

impl ToString for Comparison {
    fn to_string(&self) -> String {
        self.symbols.first().unwrap().clone()
    }
}

impl Comparison {
    pub fn new(symbols: &[&str], multi_values: bool) -> ParserResult<Comparison> {
        let symbols = symbols
            .iter()
            .map(|&sym| Self::is_valid_symbol(sym))
            .collect::<ParserResult<Vec<String>>>()?;
        if symbols.is_empty() {
            return Err(ParserError::EmptySymbol());
        }
        Ok(Comparison { symbols, multi_values })
    }

    fn is_valid_symbol(symbol: &str) -> ParserResult<String> {
        if COMP_OP_RE.is_match(symbol) || symbol == "!=" {
            Ok(symbol.to_string())
        } else {
            Err(ParserError::InvalidComparison(symbol.to_string()))
        }
    }

    #[allow(dead_code)]
    fn get_symbols(&self) -> &[String] {
        &self.symbols
    }

    #[allow(dead_code)]
    fn is_multi(&self) -> bool {
        self.multi_values
    }
}

default_comparisons! {
    EQUAL, false, ["=="];
    NOT_EQUAL, false, ["!="];
    GREATER_THAN, false, ["=gt=", ">"];
    GREATER_THAN_OR_EQUAL, false, ["=ge=", ">="];
    LESS_THAN, false, ["=lt=", "<"];
    LESS_THAN_OR_EQUAL, false, ["=le=", "<="];
    IN, true, ["=in="];
    OUT, true, ["=out="];
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
