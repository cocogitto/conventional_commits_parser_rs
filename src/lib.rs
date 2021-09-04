mod error;
pub mod commit;

#[macro_use]
extern crate pest_derive;

#[cfg(test)]
#[macro_use]
extern crate spectral;

use pest::Parser;
use crate::error::ParseError;
use crate::commit::{CommitMessage, CommitType};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ConventionalCommitParser;


pub fn parse(commit_message: &str) -> Result<CommitMessage, ParseError> {
    let pairs = ConventionalCommitParser::parse(Rule::message, commit_message)?
        .next()
        .unwrap();

    let mut commit = CommitMessage::default();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::summary => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::commit_type => {
                            let commit_type = pair.as_str();
                            let commit_type = CommitType::from(commit_type);
                            commit.commit_type = commit_type;
                        }
                        Rule::scope => {
                            let scope = pair.into_inner().next().unwrap();
                            let scope = scope.as_str();
                            if !scope.is_empty() {
                                commit.scope = Some(scope.to_string())
                            }
                        }
                        Rule::summary_content => {
                            let summary = pair.as_str();
                            commit.summary = summary.to_string();
                        }
                        Rule::breaking_change_mark => {
                            commit.is_breaking_change = true;
                        }
                        other => println!("{:?}", other)
                    }
                }
            }
            Rule::body => println!("{:?}", pair),
            Rule::footer => println!("{:?}", pair),
            _ => (),
        }
    }

    Ok(commit)
}

#[cfg(test)]
mod test {
    use crate::parse;
    use spectral::prelude::*;
    use crate::error::ParseError;
    use pest::error::{ErrorVariant, Error};
    use crate::Rule;
    use pest::Position;
    use crate::commit::CommitType;

    #[test]
    fn should_return_parse_error_when_separator_is_missing() {
        // Arrange
        let commit_message = "toto va à la plage";

        let expected_error = ParseError::MissingSeparator(
            Error::new_from_pos(
                ErrorVariant::ParsingError {
                    positives: vec![Rule::scope],
                    negatives: vec![],
                },
                Position::new("toto va à la plage", 8).unwrap(),
            ));

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_err_containing(expected_error);
    }

    #[test]
    fn should_parse_commit_type() {
        // Arrange
        let commit_message = "feat: toto va à la plage";

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::Feature);
    }

    #[test]
    fn should_parse_summary_content() {
        // Arrange
        let commit_message = "chore: version 1.0.0";

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::Chore);

        assert_that(parsed).is_ok()
            .map(|message| &message.scope)
            .is_none();

        assert_that(parsed).is_ok()
            .map(|message| &message.summary)
            .is_equal_to("version 1.0.0".to_string());
    }

    #[test]
    fn should_parse_commit_scope() {
        // Arrange
        let commit_message = "fix(parser): fix parser implementation";

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed).is_ok()
            .map(|message| &message.scope)
            .is_some()
            .is_equal_to("parser".to_string());

        assert_that(parsed).is_ok()
            .map(|message| &message.summary)
            .is_equal_to("fix parser implementation".to_string());
    }

    #[test]
    fn should_parse_exclamation_mark_breaking_change() {
        // Arrange
        let commit_message = "fix!: the fix";

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed).is_ok()
            .map(|message| &message.is_breaking_change)
            .is_true();

        assert_that(parsed).is_ok()
            .map(|message| &message.scope)
            .is_none();

        assert_that(parsed).is_ok()
            .map(|message| &message.summary)
            .is_equal_to("the fix".to_string());
    }

    #[test]
    fn should_parse_exclamation_mark_breaking_change_with_scope() {
        // Arrange
        let commit_message = "fix(the_scope)!: the fix";

        // Act
        let parsed = &parse(commit_message);


        // Assert
        assert_that(parsed).is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed).is_ok()
            .map(|message| &message.is_breaking_change)
            .is_true();

        assert_that(parsed).is_ok()
            .map(|message| &message.scope)
            .is_some()
            .is_equal_to("the_scope".to_string());

        assert_that(parsed).is_ok()
            .map(|message| &message.summary)
            .is_equal_to("the fix".to_string());
    }
}