use crate::Rule;
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseError {
    #[error("missing `:` commit type separator")]
    MissingSeparator(#[from] pest::error::Error<Rule>),
}
