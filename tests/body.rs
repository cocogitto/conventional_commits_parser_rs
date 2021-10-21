use indoc::indoc;
use spectral::prelude::*;

use conventional_commit_parser::error::ParseErrorKind;
use conventional_commit_parser::parse_body;

use crate::assertions::assert_error;

mod assertions;

#[test]
fn parsing_footer_as_body_fails() {
    // Arrange
    let body = indoc!(
        "a-token: this is a token
    another-token #this is a token with hash separator"
    )
    .to_string();

    // Act
    let result = parse_body(&body);

    // Assert
    assert_error(
        &result,
        ParseErrorKind::MalformedOrUnexpectedFooterSeparator,
    );
}

#[test]
fn parse_body_only() {
    // Arrange
    let body = indoc!(
        "A body message
    with multiple lines"
    )
    .to_string();

    // Act
    let parsed = conventional_commit_parser::parse_body(&body);

    // Assert
    assert_that(&parsed)
        .is_ok()
        .is_some()
        .is_equal_to(body.as_str());
}
