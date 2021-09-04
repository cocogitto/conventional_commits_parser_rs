use crate::commit::CommitType::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CommitType {
    Feature,
    BugFix,
    Chore,
    Revert,
    Performances,
    Documentation,
    Style,
    Refactoring,
    Test,
    Build,
    Ci,
    Custom(String),
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct CommitMessage {
    pub(crate) commit_type: CommitType,
    pub(crate) scope: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) footer: Option<String>,
    pub(crate) summary: String,
    pub(crate) is_breaking_change: bool,
}

impl From<&str> for CommitType {
    fn from(commit_type: &str) -> Self {
        match commit_type {
            "feat" => Feature,
            "fix" => BugFix,
            "chore" => Chore,
            "revert" => Revert,
            "perf" => Performances,
            "docs" => Documentation,
            "style" => Style,
            "refactor" => Refactoring,
            "test" => Test,
            "build" => Build,
            "ci" => Ci,
            other => Custom(other.to_string()),
        }
    }
}

impl Default for CommitType {
    fn default() -> Self {
        CommitType::Chore
    }
}
