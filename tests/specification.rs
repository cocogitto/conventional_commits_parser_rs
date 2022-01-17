// These parts of the specification depends on the end user and are no testable on our side

use indoc::indoc;

// 2. The type feat MUST be used when a commit adds a new feature to your application or library.
// 3. The type fix MUST be used when a commit represents a bug fix for your application.
// 11. Breaking changes MUST be indicated in the type/scope prefix of a commit, or as an entry in the footer.
// 13. If included in the type/scope prefix, breaking changes MUST be indicated by a ! immediately before the :.
//     If ! is used, BREAKING CHANGE: MAY be omitted from the footer section, and the commit description
//     SHALL be used to describe the breaking change.
// 14. Types other than feat and fix MAY be used in your commit messages, e.g., docs: updated ref docs.
use conventional_commit_parser::commit::{CommitType, Footer, Separator};
use conventional_commit_parser::error::ParseErrorKind;
use conventional_commit_parser::parse;

use crate::assertions::*;

mod assertions;

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
    assert_error(&result, ParseErrorKind::UnexpectedWhitespaceOrNewLine);
}

#[test]
fn scope_with_whitespace_should_fail() {
    // Arrange
    let commit_message = "chore(hello world): a commit";

    // Act
    let parsed = parse(commit_message);

    // Assert
    assert_error(&parsed, ParseErrorKind::UnexpectedWhitespaceOrNewLine);
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
    assert_body(&parsed, "This is a body");

    assert_contains_footer(
        parsed,
        Footer {
            token: "a-token".to_string(),
            content: "this is a token".to_string(),
            ..Default::default()
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
            token: "a-token".to_string(),
            content: "this is a token".to_string(),
            ..Default::default()
        },
    );
    assert_contains_footer(
        parsed,
        Footer {
            token: "another-token".to_string(),
            content: "this is a token with hash separator".to_string(),
            token_separator: Separator::Hash,
        },
    );
}

// A footer’s token MUST use - in place of whitespace characters, e.g., Acked-by (this helps
// differentiate the footer section from a multi-paragraph body). An exception is made for
// BREAKING CHANGE, which MAY also be used as a token.

#[test]
fn footer_with_whitespace_token_is_parsed_as_body() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    This is a body

    invalid token : this is a token"
    );

    // Act
    let result = parse(commit_message);

    // Assert
    assert_body(&result, "This is a body\n\ninvalid token : this is a token");
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
            token: "BREAKING CHANGE".to_string(),
            content: "message".to_string(),
            ..Default::default()
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
            token: "BREAKING CHANGE".to_string(),
            content: "message".to_string(),
            ..Default::default()
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
            token: "BREAKING CHANGE".to_string(),
            content: indoc!(
                "a long message that describe a footer
    with multiple new line"
            )
            .to_string(),
            ..Default::default()
        },
    );

    assert_contains_footer(
        &parsed,
        Footer {
            token: "another-footer".to_string(),
            content: "with content".to_string(),
            ..Default::default()
        },
    );

    assert_breaking_change(&parsed);
}

// 12. If included as a footer, a breaking change MUST consist of the uppercase text BREAKING CHANGE,
// followed by a colon, space, and description, e.g., BREAKING CHANGE: environment variables now take precedence over config files.
#[test]
fn lower_case_breaking_change_footer_is_parsed_as_body() {
    // Arrange
    let commit_message = indoc!(
        "chore: a commit

    the body

    breaking change: oops"
    );

    // Act
    let result = parse(commit_message);

    // Assert
    assert_body(&result, "the body\n\nbreaking change: oops");
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

    // Act
    let parsed = parse(commit_message);

    // Assert
    assert_breaking_change(&parsed);
}

#[test]
fn should_parse_dependabot_commit() {
    // Arrange
    let commit_message = indoc!(
        "chore(deps): bump spring-boot-starter-parent from 2.5.5 to 2.5.6
        Bumps [spring-boot-starter-parent](https://github.com/spring-projects/spring-boot) from 2.5.5 to 2.5.6.
        - [Release notes](https://github.com/spring-projects/spring-boot/releases)
        - [Commits](spring-projects/spring-boot@v2.5.5...v2.5.6)

        ---
        updated-dependencies:
        - dependency-name: org.springframework.boot:spring-boot-starter-parent
          dependency-type: direct:production
          update-type: version-update:semver-patch
        ...

        Signed-off-by: dependabot[bot] <support@github.com>"
    );

    // Act
    let parsed = parse(commit_message);

    // Assert
    assert_commit_type(&parsed, CommitType::Chore);
    assert_scope(&parsed, "deps");
    assert_body(&parsed, indoc!("Bumps [spring-boot-starter-parent](https://github.com/spring-projects/spring-boot) from 2.5.5 to 2.5.6.
                            - [Release notes](https://github.com/spring-projects/spring-boot/releases)
                            - [Commits](spring-projects/spring-boot@v2.5.5...v2.5.6)

                            ---"));

    assert_contains_footer(
        &parsed,
        Footer {
            token: "updated-dependencies".to_string(),
            content: indoc!(
                "- dependency-name: org.springframework.boot:spring-boot-starter-parent
                          dependency-type: direct:production
                          update-type: version-update:semver-patch
                        ..."
            )
            .to_string(),
            token_separator: Separator::ColonWithNewLine,
        },
    );

    assert_contains_footer(
        &parsed,
        Footer {
            token: "Signed-off-by".to_string(),
            content: "dependabot[bot] <support@github.com>".to_string(),
            token_separator: Separator::Colon,
        },
    );
}

#[test]
fn should_parse_dependabot_commit_2() {
    // Arrange
    let commit_message = indoc!(
        "chore(deps): bump archunit-junit5-engine from 0.21.0 to 0.22.0 (#11)

        Bumps [archunit-junit5-engine](https://github.com/TNG/ArchUnit) from 0.21.0 to 0.22.0.
        - [Release notes](https://github.com/TNG/ArchUnit/releases)
        - [Commits](https://github.com/TNG/ArchUnit/compare/v0.21.0...v0.22.0)
        
        ---
        updated-dependencies:\r- dependency-name: com.tngtech.archunit:archunit-junit5-engine
          dependency-type: direct:production
          update-type: version-update:semver-minor
        ...
        
        Signed-off-by: dependabot[bot] <support@github.com>
        
        Co-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>
        Co-authored-by: guillaumer63 <74533647+guillaumer63@users.noreply.github.com>"
    );

    // Act
    let parsed = parse(commit_message);

    // Assert
    assert_commit_type(&parsed, CommitType::Chore);
    assert_scope(&parsed, "deps");
    assert_body(&parsed, indoc!(
        "Bumps [archunit-junit5-engine](https://github.com/TNG/ArchUnit) from 0.21.0 to 0.22.0.
        - [Release notes](https://github.com/TNG/ArchUnit/releases)
        - [Commits](https://github.com/TNG/ArchUnit/compare/v0.21.0...v0.22.0)
        
        ---"));

    assert_contains_footer(
        &parsed,
        Footer {
            token: "updated-dependencies".to_string(),
            content: indoc!(
                "- dependency-name: com.tngtech.archunit:archunit-junit5-engine
                  dependency-type: direct:production
                  update-type: version-update:semver-minor
                ..."
            )
            .to_string(),
            token_separator: Separator::ColonWithNewLine,
        },
    );

    assert_contains_footer(
        &parsed,
        Footer {
            token: "Signed-off-by".to_string(),
            content: "dependabot[bot] <support@github.com>".to_string(),
            token_separator: Separator::Colon,
        },
    );
    assert_contains_footer(
        &parsed,
        Footer {
            token: "Co-authored-by".to_string(),
            content: "guillaumer63 <74533647+guillaumer63@users.noreply.github.com>".to_string(),
            token_separator: Separator::Colon,
        },
    );
}

#[test]
fn should_parse_interactively_rebased_commit() {
    // Arrange
    let commit_message = indoc!(
        "feat(References): add new fields in ReferenceWithAlert (#8)

        * feat(References): add new fields in ReferenceWithAlert

        * feat(PortOfLoading): add unit test

        footer: value"
    );

    // Act
    let parsed = parse(commit_message);

    // Assert
    assert_commit_type(&parsed, CommitType::Feature);
    assert_scope(&parsed, "References");
    assert_body(
        &parsed,
        indoc!(
            "* feat(References): add new fields in ReferenceWithAlert

        * feat(PortOfLoading): add unit test"
        ),
    );

    assert_contains_footer(
        &parsed,
        Footer {
            token: "footer".to_string(),
            content: "value".to_string(),
            token_separator: Separator::Colon,
        },
    );
}
