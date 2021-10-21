// These parts of the specification depends on the end user and are no testable on our side

// 2. The type feat MUST be used when a commit adds a new feature to your application or library.
// 3. The type fix MUST be used when a commit represents a bug fix for your application.
// 11. Breaking changes MUST be indicated in the type/scope prefix of a commit, or as an entry in the footer.
// 13. If included in the type/scope prefix, breaking changes MUST be indicated by a ! immediately before the :.
//     If ! is used, BREAKING CHANGE: MAY be omitted from the footer section, and the commit description
//     SHALL be used to describe the breaking change.
// 14. Types other than feat and fix MAY be used in your commit messages, e.g., docs: updated ref docs.
use conventional_commit_parser::commit::{CommitType, Footer};
use conventional_commit_parser::parse;
use indoc::indoc;

mod assertions;

use crate::assertions::*;
use conventional_commit_parser::error::ParseErrorKind;

// 1. Commits MUST be prefixed with a type, which consists of a noun, feat, fix, etc., followed by
// the OPTIONAL scope, OPTIONAL !, and REQUIRED terminal colon and space.

// 5. A description MUST immediately follow the colon and space after the type/scope prefix.
// The description is a short summary of the code changes, e.g., fix: array parsing issue when multiple spaces were contained in string.

#[test]
fn commits_with_feature_type() {
    // Arrange
    let commit_message = "feat: toto va à la plage";

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_commit_type(parsed, CommitType::Feature);
    assert_summary(parsed, "toto va à la plage");
    assert_no_body(parsed);
    assert_no_footers(parsed);
    assert_not_breaking_change(parsed);
}

#[test]
fn parsing_a_commit_type_without_colon_separator_should_fail() {
    // Arrange
    let commit_message = "feat toto va à la plage";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MissingSeparator);
}

#[test]
fn parsing_a_commit_type_with_whitespace_should_fail() {
    // Arrange
    let commit_message = "feat toto: va à la plage";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MissingSeparator);
}

#[test]
fn commits_with_feature_type_and_breaking_change_mark() {
    // Arrange
    let commit_message = "feat!: toto va à la plage";

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_commit_type(parsed, CommitType::Feature);
    assert_no_scope(parsed);
    assert_summary(parsed, "toto va à la plage");
    assert_breaking_change(parsed);

    assert_no_body(parsed);
    assert_no_footers(parsed);
}

#[test]
fn commits_with_feature_type_and_scope_and_breaking_change_mark() {
    // Arrange
    let commit_message = "fix(parser)!: the parser";

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_commit_type(parsed, CommitType::BugFix);
    assert_scope(parsed, "parser");
    assert_summary(parsed, "the parser");
    assert_breaking_change(parsed);

    assert_no_body(parsed);
    assert_no_footers(parsed);
}

#[test]
fn parsing_a_commit_type_without_terminal_column_and_space_should_fail() {
    // Arrange
    let commit_message = "feat:toto va à la plage";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MissingWhiteSpace);
}

#[test]
fn parsing_a_scoped_commit_type_without_terminal_column_and_space_should_fail() {
    // Arrange
    let commit_message = "feat(toto):toto va à la plage";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MissingWhiteSpace);
}

#[test]
fn parsing_a_scoped_breaking_change_commit_type_without_terminal_column_and_space_should_fail() {
    // Arrange
    let commit_message = "feat(toto)!:toto va à la plage";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MissingWhiteSpace);
}

// 3. A scope MAY be provided after a type. A scope MUST consist of a noun describing a section of
// the codebase surrounded by parenthesis, e.g., fix(parser):
#[test]
fn commits_with_scope() {
    // Arrange
    let commit_message = "fix(parser): the parser";

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_scope(parsed, "parser");
}

#[test]
fn scope_with_inner_parenthesis_should_fail() {
    // Arrange
    let commit_message = "fix((toto): the parser";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::UnexpectedParenthesis);
}

#[test]
fn scope_with_inner_new_line_should_fail() {
    // Arrange
    let commit_message = "fix(\n)): the parser";

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(&result, ParseErrorKind::MalformedScope);
}

// 6. A longer commit body MAY be provided after the short description, providing additional contextual
// information about the code changes. The body MUST begin one blank line after the description.
// 7. A commit body is free-form and MAY consist of any number of newline separated paragraphs.
#[test]
fn commits_with_body() {
    // Arrange
    let commit_message = indoc!(
        "ci(the_scope)!: the fix

    This is a body containing special char like / and \\ and also
    Newline. Punctuation and special chars ? , ; ...
    Number is something you can have to ! 1 2 .. 42"
    );

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_body(
        parsed,
        indoc!(
            "This is a body containing special char like / and \\ and also
            Newline. Punctuation and special chars ? , ; ...
            Number is something you can have to ! 1 2 .. 42"
        ),
    );

    assert_no_footers(parsed);
}

// 7. One or more footers MAY be provided one blank line after the body. Each footer MUST consist of
// a word token, followed by either a :<space> or <space># separator, followed by a string value
// (this is inspired by the git trailer convention).
#[test]
fn commits_with_footer() {
    // Arrange
    let commit_message = indoc!(
        "feat(friture): the the BIG feature

    This is a body

    a-token: this is a token"
    );

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_contains_footer(
        parsed,
        Footer {
            token: "a-token",
            content: "this is a token",
        },
    );
}

#[test]
fn commits_with_footers() {
    // Arrange
    let commit_message = indoc!(
        "feat(friture): the the BIG feature

    This is a body

    a-token: this is a token
    another-token #this is a token with hash separator"
    );

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_contains_footer(
        parsed,
        Footer {
            token: "a-token",
            content: "this is a token",
        },
    );
    assert_contains_footer(
        parsed,
        Footer {
            token: "another-token",
            content: "this is a token with hash separator",
        },
    );
}

// A footer’s token MUST use - in place of whitespace characters, e.g., Acked-by (this helps
// differentiate the footer section from a multi-paragraph body). An exception is made for
// BREAKING CHANGE, which MAY also be used as a token.

#[test]
fn footer_with_whitespace_token_fail() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    This is a body

    invalid token : this is a token"
    );

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(
        &result,
        ParseErrorKind::MalformedOrUnexpectedFooterSeparator,
    );
}

#[test]
fn footer_with_breaking_change_ok() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    This is a body

    BREAKING CHANGE: message"
    );

    let parsed = parse(commit_message);

    assert_contains_footer(
        &parsed,
        Footer {
            token: "BREAKING CHANGE",
            content: "message",
        },
    );

    assert_breaking_change(&parsed);
}

#[test]
fn footer_with_no_body() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    BREAKING CHANGE: message"
    );

    let parsed = parse(commit_message);

    assert_no_body(&parsed);
    assert_contains_footer(
        &parsed,
        Footer {
            token: "BREAKING CHANGE",
            content: "message",
        },
    );

    assert_breaking_change(&parsed);
}

// A footer’s value MAY contain spaces and newlines, and parsing MUST terminate when the next valid
// footer token/separator pair is observed.
#[test]
fn footer_with_new_line() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    BREAKING CHANGE: a long message that describe a footer
    with multiple new line
    another-footer: with content"
    );

    let parsed = parse(commit_message);

    assert_no_body(&parsed);
    assert_contains_footer(
        &parsed,
        Footer {
            token: "BREAKING CHANGE",
            content: indoc!(
                "a long message that describe a footer
    with multiple new line"
            )
            ,
        },
    );

    assert_contains_footer(
        &parsed,
        Footer {
            token: "another-footer",
            content: "with content",
        },
    );

    assert_breaking_change(&parsed);
}

// 12. If included as a footer, a breaking change MUST consist of the uppercase text BREAKING CHANGE,
// followed by a colon, space, and description, e.g., BREAKING CHANGE: environment variables now take precedence over config files.
#[test]
fn lower_case_breaking_change_footer_fails() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    the body

    breaking change: oops"
    );

    // Act
    let result = parse(commit_message);

    // Assert
    assert_error(
        &result,
        ParseErrorKind::MalformedOrUnexpectedFooterSeparator,
    );
}

// 15. The units of information that make up Conventional Commits MUST NOT be treated as case sensitive
// by implementors, with the exception of BREAKING CHANGE which MUST be uppercase.
#[test]
fn commits_with_camel_case_feature_type() {
    // Arrange
    let commit_message = "Feat: toto va à la plage";

    // Act
    let parsed = &parse(commit_message);

    // Assert
    assert_commit_type(parsed, CommitType::Feature);
}

// 16. BREAKING-CHANGE MUST be synonymous with BREAKING CHANGE, when used as a token in a footer.
#[test]
fn breaking_change_with_dash() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    BREAKING-CHANGE: message"
    );

    let parsed = parse(commit_message);

    assert_breaking_change(&parsed);
}
