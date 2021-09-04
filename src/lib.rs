pub mod commit;
mod error;

#[macro_use]
extern crate pest_derive;

#[cfg(test)]
#[macro_use]
extern crate spectral;

#[cfg(test)]
#[macro_use]
extern crate indoc;

use crate::commit::CommitMessage;
use crate::error::ParseError;
use pest::Parser;

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
                        Rule::commit_type => commit.set_commit_type(&pair),
                        Rule::scope => commit.set_scope(pair),
                        Rule::summary_content => commit.set_summary_content(pair),
                        Rule::breaking_change_mark => commit.set_breaking_change(),
                        other => println!("{:?}", other),
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

#[cfg(test)]
mod test {
    use crate::commit::{CommitType, Footer};
    use crate::error::ParseError;
    use crate::parse;
    use crate::Rule;
    use pest::error::{Error, ErrorVariant};
    use pest::Position;
    use spectral::prelude::*;

    #[test]
    fn should_return_parse_error_when_separator_is_missing() {
        // Arrange
        let commit_message = "toto va à la plage";

        let expected_error = ParseError::MissingSeparator(Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![Rule::separator],
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
        assert_that(parsed)
            .is_ok()
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
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::Chore);

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.scope)
            .is_none();

        assert_that(parsed)
            .is_ok()
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
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.scope)
            .is_some()
            .is_equal_to("parser".to_string());

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.summary)
            .is_equal_to("fix parser implementation".to_string());
    }

    #[test]
    fn should_fail_to_parse_scope_with_whitespaces() {
        // Arrange
        let commit_message = "fix(the parser): fix parser implementation";

        let expected_error = ParseError::MissingSeparator(Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![Rule::scope],
                negatives: vec![],
            },
            Position::new("fix(the parser): fix parser implementation", 3).unwrap(),
        ));

        // Act
        let parsed = &parse(commit_message);

        // Assert
        assert_that(parsed).is_err_containing(expected_error)
    }

    #[test]
    fn should_parse_exclamation_mark_breaking_change() {
        // Arrange
        let commit_message = "fix!: the fix";

        // Act
        let parsed = &parse(commit_message);

        // Assert
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.is_breaking_change)
            .is_true();

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.scope)
            .is_none();

        assert_that(parsed)
            .is_ok()
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
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.commit_type)
            .is_equal_to(CommitType::BugFix);

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.is_breaking_change)
            .is_true();

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.scope)
            .is_some()
            .is_equal_to("the_scope".to_string());

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.summary)
            .is_equal_to("the fix".to_string());
    }

    #[test]
    fn should_parse_commit_body() {
        // Arrange
        let commit_message = indoc!(
            "fix(the_scope)!: the fix

        This is a body containing special char like / and \\ and also
        Newline. Punctuation and special chars: ? , ; : # ...
        Number is something you can have to ! 1 2 .. 42"
        );

        let expected_body = indoc!(
            "This is a body containing special char like / and \\ and also
        Newline. Punctuation and special chars: ? , ; : # ...
        Number is something you can have to ! 1 2 .. 42"
        )
        .to_string();

        // Act
        let parsed = &parse(commit_message);

        // Assert
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.body)
            .is_some()
            .is_equal_to(expected_body);
    }

    #[test]
    fn should_parse_commit_footer() {
        // Arrange
        let commit_message = indoc!(
            "fix(the_scope)!: the fix

        This is a body containing special char like / and \\ and also
        Newline. Punctuation and special chars: ? , ; : # ...
        Number is something you can have to ! 1 2 .. 42

        Acked-by: okno
        Release: The message"
        );

        // Act
        let parsed = &parse(commit_message);

        // Assert
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.footers)
            .contains(Footer {
                token: "Acked-by".to_string(),
                content: "okno".to_string(),
            });

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.footers)
            .contains(Footer {
                token: "Release".to_string(),
                content: "The message".to_string(),
            });
    }

    #[test]
    fn should_parse_commit_footer_with_hash() {
        // Arrange
        let commit_message = indoc!(
            "fix(the_scope)!: the fix

        This is a body containing special char like / and \\ and also
        Newline. Punctuation and special chars: ? , ; : # ...
        Number is something you can have to ! 1 2 .. 42

        Acked-by #okno
        Release #The message"
        );

        // Act
        let parsed = &parse(commit_message);

        // Assert
        assert_that(parsed)
            .is_ok()
            .map(|message| &message.footers)
            .contains(Footer {
                token: "Acked-by".to_string(),
                content: "okno".to_string(),
            });

        assert_that(parsed)
            .is_ok()
            .map(|message| &message.footers)
            .contains(Footer {
                token: "Release".to_string(),
                content: "The message".to_string(),
            });
    }
}
