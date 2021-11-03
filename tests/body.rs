use indoc::indoc;
use speculoos::prelude::*;

mod assertions;

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
    assert_that(&parsed).is_ok().is_some().is_equal_to(body);
}
