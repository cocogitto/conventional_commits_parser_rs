use conventional_commit_parser::parse_summary;

mod assertions;
use assertions::*;
use conventional_commit_parser::commit::CommitType;

#[test]
fn parse_summary_only() {
    // Arrange
    let summary = "feat(scope): message";

    // Act
    let result = parse_summary(summary);

    // Assert
    assert_scope(&result, "scope");
    assert_commit_type(&result, CommitType::Feature);
    assert_summary(&result, "message");
    assert_no_footers(&result);
    assert_no_body(&result);
}
