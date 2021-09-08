#[macro_use]
extern crate pest_derive;

#[cfg(test)]
#[macro_use]
extern crate spectral;

use anyhow::Result;
use pest::Parser;

use crate::commit::CommitMessage;
use crate::error::ParseError;

pub mod commit;
pub mod error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ConventionalCommitParser;

pub fn parse(commit_message: &str) -> Result<CommitMessage> {
    let pairs = ConventionalCommitParser::parse(Rule::message, commit_message)
        .map_err(ParseError::from)?
        .next()
        .unwrap();

    let mut commit = CommitMessage::default();

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
