use thiserror::Error;
use crate::Rule;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseError {
    #[error("missing `:` commit type separator")]
    MissingSeparator(#[from] pest::error::Error<Rule>),
}