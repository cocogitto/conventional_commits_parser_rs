use conventional_commit_parser::commit::Footer;
use indoc::indoc;
use spectral::prelude::*;

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
            token: "a-token",
            content: "this is a token",
        },
        &Footer {
            token: "another-token",
            content: "this is a token with hash separator",
        },
    ]);
}
