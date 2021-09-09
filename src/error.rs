use crate::Rule;
use pest::error::Error as PestError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    MissingSeparator(ParseErrorHelper),
    MissingWhiteSpaceAfterSeparator(ParseErrorHelper),
    MalformedScope(ParseErrorHelper),
    MalformedTokenFooter(ParseErrorHelper),
    Other(ParseErrorHelper),
}

#[derive(Debug)]
pub struct ParseErrorHelper {
    pub message: String,
    pub error: PestError<Rule>,
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::MissingSeparator(pest_error)
            | ParseError::MissingWhiteSpaceAfterSeparator(pest_error)
            | ParseError::MalformedScope(pest_error)
            | ParseError::MalformedTokenFooter(pest_error)
            | ParseError::Other(pest_error) => Some(&pest_error.error),
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pest_err = match self {
            ParseError::MissingSeparator(e)
            | ParseError::MissingWhiteSpaceAfterSeparator(e)
            | ParseError::MalformedTokenFooter(e)
            | ParseError::Other(e)
            | ParseError::MalformedScope(e) => e,
        };

        write!(f, "{}", pest_err.message)
    }
}

impl From<PestError<Rule>> for ParseError {
    fn from(pest_error: PestError<Rule>) -> Self {
        match pest_error.variant {
            pest::error::ErrorVariant::ParsingError { ref positives, .. } => {
                if positives.contains(&Rule::type_separator) {
                    ParseError::MissingSeparator(ParseErrorHelper {
                        message: "Missing `:` after commit type".to_string(),
                        error: pest_error.clone(),
                    })
                } else if positives.contains(&Rule::whitespace_terminal) {
                    ParseError::MissingWhiteSpaceAfterSeparator(ParseErrorHelper {
                        message: "Missing whitespace terminal after commit type separator `:`"
                            .to_string(),
                        error: pest_error.clone(),
                    })
                } else if positives.contains(&Rule::scope_content) {
                    ParseError::MissingWhiteSpaceAfterSeparator(ParseErrorHelper {
                        message: "Malformed scope".to_string(),
                        error: pest_error.clone(),
                    })
                } else if positives.contains(&Rule::breaking_change_token)
                    || positives.contains(&Rule::token_separator)
                {
                    ParseError::MissingWhiteSpaceAfterSeparator(ParseErrorHelper {
                        message: "Malformed footer token".to_string(),
                        error: pest_error.clone(),
                    })
                } else {
                    ParseError::Other(ParseErrorHelper {
                        message: "Unexpected parsing error".to_string(),
                        error: pest_error.clone(),
                    })
                }
            }
            pest::error::ErrorVariant::CustomError { .. } => ParseError::Other(ParseErrorHelper {
                message: "Custom error".to_string(),
                error: pest_error.clone(),
            }),
        }
    }
}
