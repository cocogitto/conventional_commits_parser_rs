use crate::commit::CommitType::*;
use crate::Rule;
use pest::iterators::Pair;

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
pub struct Footer {
    pub token: String,
    pub content: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CommitMessage {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub body: Option<String>,
    pub footers: Vec<Footer>,
    pub summary: String,
    pub is_breaking_change: bool,
}

impl Default for CommitMessage {
    fn default() -> Self {
        CommitMessage {
            commit_type: Feature,
            scope: None,
            body: None,
            footers: vec![],
            summary: "".to_string(),
            is_breaking_change: false,
        }
    }
}

impl CommitMessage {
    pub(crate) fn set_breaking_change(&mut self, pair: Pair<Rule>) {
        if !pair.as_str().is_empty() {
            self.is_breaking_change = true
        }
    }

    pub(crate) fn set_summary_content(&mut self, pair: Pair<Rule>) {
        let summary = pair.as_str();
        self.summary = summary.to_string();
    }

    pub(crate) fn set_scope(&mut self, pair: Pair<Rule>) {
        if let Some(scope) = pair.into_inner().next() {
            let scope = scope.as_str();
            if !scope.is_empty() {
                self.scope = Some(scope.to_string())
            }
        };
    }

    pub(crate) fn set_commit_type(&mut self, pair: &Pair<Rule>) {
        let commit_type = pair.as_str();
        let commit_type = CommitType::from(commit_type);
        self.commit_type = commit_type;
    }

    pub(crate) fn set_commit_body(&mut self, pair: Pair<Rule>) {
        if let Some(body) = pair.into_inner().next() {
            let body = body.as_str();
            if !body.is_empty() {
                self.body = Some(body.to_string())
            }
        };
    }

    pub(crate) fn set_footers(&mut self, pair: Pair<Rule>) {
        for footer in pair.into_inner() {
            self.set_footer(footer);
        }
    }

    fn set_footer(&mut self, footer: Pair<Rule>) {
        let mut footer_pairs = footer.into_inner();
        let token = footer_pairs.next().unwrap().as_str().to_string();

        if token == "BREAKING CHANGE" || token == "BREAKING-CHANGE" {
            self.is_breaking_change = true;
        }

        let _separator = footer_pairs.next().unwrap();
        let content = footer_pairs.next().unwrap().as_str().to_string();

        self.footers.push(Footer { token, content });
    }
}

impl From<&str> for CommitType {
    fn from(commit_type: &str) -> Self {
        match commit_type.to_ascii_lowercase().as_str() {
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
