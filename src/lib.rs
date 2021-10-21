//! # Conventional commit parser
//!
//! A rust implementation of the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/).
//! This crate expose functions to parse conventional commit messages.
//!
//! ## Example :
//! ```
//! # use conventional_commit_parser::error::ParseError;
//! # fn main() -> Result<(), ParseError> {
//!
//! use conventional_commit_parser::parse;
//! use conventional_commit_parser::commit::*;
//! let message = r#"fix: correct minor typos in code
//!
//! see the issue for details
//!
//! on typos fixed.
//!
//! Reviewed-by: Z
//! Refs #133"#;
//!
//! let commit = parse(message)?;
//!
//! assert_eq!(commit.commit_type, CommitType::BugFix);
//! assert_eq!(commit.summary, "correct minor typos in code");
//! assert_eq!(commit.body, Some(r#"see the issue for details
//!
//! on typos fixed."#));
//!
//! assert_eq!(commit.footers, vec![
//!     Footer {token: "Reviewed-by", content: "Z"},
//!     Footer {token: "Refs", content: "133",}
//! ]);
//!
//! # Ok(())
//! # }
//! ```
//!
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
#[macro_use]
extern crate spectral;

use pest::Parser;

use crate::commit::{ConventionalCommit, Footer};
use crate::error::ParseError;

/// Conventional commit representation, produced by the [parse] function
///
/// [parse]: crate::ConventionalCommitParser::parse
pub mod commit;

pub mod error;

#[doc(hidden)]
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ConventionalCommitParser;

/// Parse a commit message into a [`commit::ConventionalCommit`]
pub fn parse(commit_message: &str) -> Result<ConventionalCommit, ParseError> {
    let pairs = ConventionalCommitParser::parse(Rule::message, commit_message)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let mut commit = ConventionalCommit::default();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::summary => commit.set_summary(pair),
            Rule::body => commit.set_commit_body(pair),
            Rule::footers => commit.set_footers(pair),
            _ => (),
        }
    }

    Ok(commit)
}

/// Parse a commit summary of the following form : `<type>[optional scope]: <description>`
/// Returns a [`ConventionalCommit`] struct with a `None` body and empty footers.
///
/// # Example :
/// ```
/// # use conventional_commit_parser::error::ParseError;
/// # fn main() -> Result<(), ParseError> {
///
/// use conventional_commit_parser::parse_summary;
/// use conventional_commit_parser::commit::*;
///
/// let message = "feat(parser): implement parse_summary";
///
/// let parsed = parse_summary(message).expect("Parse error");
///
/// assert_eq!(parsed, ConventionalCommit {
///     commit_type: CommitType::Feature,
///     scope: Some("parser"),
///     summary: "implement parse_summary",
///     body: None,
///     footers: vec![],
///     is_breaking_change: false
/// });
/// # Ok(())
/// # }
pub fn parse_summary(summary: &str) -> Result<ConventionalCommit, ParseError> {
    let pair = ConventionalCommitParser::parse(Rule::summary, summary)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let mut commit = ConventionalCommit::default();
    commit.set_summary(pair);

    Ok(commit)
}

/// Parse a commit body only returning an `Option<String>` on a non empty trimmed value
///
/// # Example :
/// ```
/// # use conventional_commit_parser::error::ParseError;
/// # fn main() -> Result<(), ParseError> {
///
/// use conventional_commit_parser::parse_body;
/// use conventional_commit_parser::commit::*;
///
/// let body = r#"resolves [#10]
/// will be merged in the next release"#;
///
/// let parsed = parse_body(body).expect("Parse error");
///
/// assert_eq!(parsed, Some(body.to_string()));
/// # Ok(())
/// # }
pub fn parse_body(body: &str) -> Result<Option<String>, ParseError> {
    let pair = ConventionalCommitParser::parse(Rule::body, body)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let body = pair.as_str();
    if !body.is_empty() {
        Ok(Some(body.to_string()))
    } else {
        Ok(None)
    }
}

/// Parse commit footers only
///
/// # Example :
/// ```
/// # use conventional_commit_parser::error::ParseError;
/// # fn main() -> Result<(), ParseError> {
///
/// use conventional_commit_parser::parse_footers;
/// use conventional_commit_parser::commit::*;
///
/// let footer = r#"a-token: this is a token
/// another-token #this is a token with hash separator"#;
///
/// let parsed = parse_footers(footer).expect("Parse error");
///
/// assert_eq!(parsed, vec![
///     Footer { token: "a-token", content: "this is a token" },
///     Footer { token: "another-token", content: "this is a token with hash separator" }
/// ]);
/// # Ok(())
/// # }
pub fn parse_footers(footers: &str) -> Result<Vec<Footer>, ParseError> {
    let pair = ConventionalCommitParser::parse(Rule::footers, footers)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let mut footers = vec![];
    for pair in pair.into_inner() {
        footers.push(Footer::from(pair));
    }

    Ok(footers)
}
