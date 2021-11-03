use conventional_commit_parser::commit::{Footer, Separator};
use indoc::indoc;
use speculoos::prelude::*;

#[test]
pub fn parse_footer_only() {
    // Arrange
    let footers = indoc!(
        "a-token: this is a token
        another-token #this is a token with hash separator"
    );

    // Act
    let parsed = conventional_commit_parser::parse_footers(footers);

    // Assert
    assert_that(&parsed).is_ok().contains_all_of(&vec![
        &Footer {
            token: "a-token".to_string(),
            content: "this is a token".to_string(),
            ..Default::default()
        },
        &Footer {
            token: "another-token".to_string(),
            content: "this is a token with hash separator".to_string(),
            token_separator: Separator::Hash,
        },
    ]);
}
