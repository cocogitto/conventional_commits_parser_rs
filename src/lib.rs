//! # Conventional commit parser
//!
//! A rust implementation of the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/).
//! This crate provides a single [parse] function returning a [`ConventionalCommit`].
//!
//! [parse]: crate::ConventionalCommitParser::parse
//! ```
//! # fn main() -> anyhow::Result<()> {
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
//! let conventional_commit = parse(message)?;
//!
//! assert_eq!(conventional_commit.commit_type, CommitType::BugFix);
//! assert_eq!(conventional_commit.summary, "correct minor typos in code".to_string());
//! assert_eq!(conventional_commit.body, Some(r#"see the issue for details
//!
//! on typos fixed."#.to_string()));
//!
//! assert_eq!(conventional_commit.footers, vec![
//!     Footer {token: "Reviewed-by".to_string(), content: "Z".to_string()},
//!     Footer {token: "Refs".to_string(), content: "133".to_string(),}
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

use anyhow::Result;
use pest::Parser;

use crate::commit::ConventionalCommit;
use crate::error::ParseError;

/// Conventional commit representation, produced by the [parse] function
///
/// [parse]: crate::ConventionalCommitParser::parse
pub mod commit;

/// Parse errors, note that parse results are wrapped in [anyhow::Result](https://docs.rs/anyhow/1.0.43/anyhow/type.Result.html)
pub mod error;

#[doc(hidden)]
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ConventionalCommitParser;

/// Parse a commit message into a [`commit::ConventionalCommit`]
pub fn parse(commit_message: &str) -> Result<ConventionalCommit> {
    let pairs = ConventionalCommitParser::parse(Rule::message, commit_message)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let mut commit = ConventionalCommit::default();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::summary => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::commit_type => commit.set_commit_type(&pair),
                        Rule::scope => commit.set_scope(pair),
                        Rule::summary_content => commit.set_summary_content(pair),
                        Rule::breaking_change_mark => commit.set_breaking_change(pair),
                        _other => (),
                    }
                }
            }
            Rule::body => commit.set_commit_body(pair),
            Rule::footers => commit.set_footers(pair),
            _ => (),
        }
    }

    Ok(commit)
}
