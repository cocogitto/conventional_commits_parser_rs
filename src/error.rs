use crate::Rule;
use pest::error::Error as PestError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ParseError {
    pub inner: PestError<Rule>,
    pub kind: ParseErrorKind,
}

/// Common conventional commit formatting errors are wrapped in this struct to produce an additional hint
#[derive(Debug, PartialEq)]
pub enum ParseErrorKind {
    MissingSeparator,
    MissingWhiteSpace,
    UnexpectedParenthesis,
    MalformedScope,
    MalformedOrUnexpectedFooterSeparator,
    Other,
}

impl AsRef<str> for ParseErrorKind {
    fn as_ref(&self) -> &str {
        match &self {
            ParseErrorKind::MissingSeparator => "Missing commit type separator `:`",
            ParseErrorKind::MissingWhiteSpace => {
                "Missing whitespace terminal after commit type separator `:`"
            }
            ParseErrorKind::UnexpectedParenthesis => {
                "A scope value cannot contains inner parenthesis"
            }
            ParseErrorKind::MalformedScope => "Malformed commit scope",
            ParseErrorKind::MalformedOrUnexpectedFooterSeparator => {
                "Either token separator (` #` or `: `) \
            \nis missing from the footer or a footer was not expected at this point"
            }
            ParseErrorKind::Other => "Parse error",
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.as_ref())
    }
}

impl From<PestError<Rule>> for ParseError {
    fn from(pest_error: PestError<Rule>) -> Self {
        let kind = match pest_error.variant {
            pest::error::ErrorVariant::ParsingError { ref positives, .. } => {
                if positives.contains(&Rule::type_separator) {
                    ParseErrorKind::MissingSeparator
                } else if positives.contains(&Rule::unexpected_parenthesis) {
                    ParseErrorKind::UnexpectedParenthesis
                } else if positives.contains(&Rule::whitespace_terminal) {
                    ParseErrorKind::MissingWhiteSpace
                } else if positives.contains(&Rule::scope_content) {
                    ParseErrorKind::MalformedScope
                } else if positives.contains(&Rule::token_separator) {
                    ParseErrorKind::MalformedOrUnexpectedFooterSeparator
                } else {
                    ParseErrorKind::Other
                }
            }
            pest::error::ErrorVariant::CustomError { .. } => ParseErrorKind::Other,
        };

        ParseError {
            inner: pest_error,
            kind,
        }
    }
}
