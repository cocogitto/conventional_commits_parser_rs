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

// 10. A footer’s value MAY contain spaces and newlines, and parsing MUST terminate when the next valid footer token/separator pair is observed.
#[test]
pub fn parse_footer_with_new_lines() {
    // Arrange
    let footers = indoc!(
        "updated-dependencies:
                         - dependency-name: org.springframework.boot:spring-boot-starter-parent
                           dependency-type: direct:production
                           update-type: version-update:semver-patch
                         ..."
    );

    // Act
    let parsed = conventional_commit_parser::parse_footers(footers);

    // Assert
    assert_that(&parsed).is_ok().contains_all_of(&vec![&Footer {
        token: "updated-dependencies".to_string(),
        content: indoc!(
            "- dependency-name: org.springframework.boot:spring-boot-starter-parent
                               dependency-type: direct:production
                               update-type: version-update:semver-patch
                             ..."
        )
        .to_string(),
        token_separator: Separator::ColonWithNewLine,
    }]);
}
