use crate::commit::CommitType::*;
use crate::Rule;
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Formatter;

/// A commit type consist of a noun describing the kind of modification made.
/// In addition to the mandatory `fix` and `feat` type, common commit types taken from
/// [the angular convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)
/// as their own enum variant. Other type will be parser as [`CommitType::Custom`]
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CommitType {
    /// *feat*: a commit of the type `feat` introduces a new feature to the codebase (this correlates with `MINOR` in Semantic Versioning).
    Feature,
    /// *fix*: a commit of the type `fix` patches a bug in your codebase (this correlates with `PATCH` in Semantic Versioning).
    BugFix,
    /// *chore*: Miscellaneous chores
    Chore,
    /// See [How does Conventional Commits handle revert commits?](https://www.conventionalcommits.org/en/v1.0.0/#how-does-conventional-commits-handle-revert-commits)
    Revert,
    /// *perf*: A code change that improves performance
    Performances,
    /// *docs*: Documentation only changes
    Documentation,
    /// *style*: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
    Style,
    /// *refactor*: A code change that neither fixes a bug nor adds a feature
    Refactor,
    /// *test*: Adding missing tests or correcting existing tests
    Test,
    /// *build*: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
    Build,
    /// *ci*: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
    Ci,
    /// A custom commit type, can be anything
    Custom(String),
}

/// One or more footers MAY be provided one blank line after the body. Each footer MUST consist of
/// a word token, followed by either a :<space> or <space># separator, followed by a string value.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Footer {
    /// The footer token, either BREAKING CHANGE or a work token
    pub token: String,
    /// A string value holding the footer message
    pub content: String,
}

impl Footer {
    /// Return true if a footer as the breaking change token
    /// ```rust
    /// # fn main() {
    /// use conventional_commit_parser::commit::Footer;
    /// use std::ops::Not;
    /// let footer = Footer {
    ///     token: "BREAKING CHANGE".to_string(),content: "some changes were made".to_string(),
    /// };
    ///
    /// assert!(footer.is_breaking_change());
    ///
    /// let footer = Footer {
    ///     token: "a-token".to_string(),content: "Ref 133".to_string(),
    /// };
    ///
    /// assert!(footer.is_breaking_change().not());
    /// # }
    pub fn is_breaking_change(&self) -> bool {
        self.token == "BREAKING CHANGE" || self.token == "BREAKING-CHANGE"
    }
}

/// A conventional commit compliant commit message produced by the [parse] function
///
/// [parse]: crate::ConventionalCommitParser::parse
#[derive(Debug, Eq, PartialEq)]
pub struct ConventionalCommit {
    /// The commit type, `fix`, `feat` etc.
    pub commit_type: CommitType,
    /// An optional scope
    pub scope: Option<String>,
    /// Commit description summary
    pub summary: String,
    /// An optional commit body
    pub body: Option<String>,
    /// A list of commit  footers
    pub footers: Vec<Footer>,
    /// A commit that has a footer `BREAKING CHANGE` or a `!` after the commit type and scope
    pub is_breaking_change: bool,
}

impl From<Pair<'_, Rule>> for Footer {
    fn from(pairs: Pair<'_, Rule>) -> Self {
        let mut pair = pairs.into_inner();
        let token = pair.next().unwrap().as_str().to_string();
        let _separator = pair.next().unwrap();
        let content = pair.next().unwrap().as_str().to_string();

        Footer { token, content }
    }
}

impl Default for ConventionalCommit {
    fn default() -> Self {
        ConventionalCommit {
            commit_type: Feature,
            scope: None,
            body: None,
            footers: vec![],
            summary: "".to_string(),
            is_breaking_change: false,
        }
    }
}

impl ConventionalCommit {
    pub(crate) fn set_summary(&mut self, pair: Pair<Rule>) {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::commit_type => self.set_commit_type(&pair),
                Rule::scope => self.set_scope(pair),
                Rule::summary_content => self.set_summary_content(pair),
                Rule::breaking_change_mark => self.set_breaking_change(pair),
                _other => (),
            }
        }
    }

    fn set_breaking_change(&mut self, pair: Pair<Rule>) {
        if !pair.as_str().is_empty() {
            self.is_breaking_change = true
        }
    }

    fn set_summary_content(&mut self, pair: Pair<Rule>) {
        let summary = pair.as_str();
        self.summary = summary.to_string();
    }

    fn set_scope(&mut self, pair: Pair<Rule>) {
        if let Some(scope) = pair.into_inner().next() {
            let scope = scope.as_str();
            if !scope.is_empty() {
                self.scope = Some(scope.to_string())
            }
        };
    }

    pub fn set_commit_type(&mut self, pair: &Pair<Rule>) {
        let commit_type = pair.as_str();
        let commit_type = CommitType::from(commit_type);
        self.commit_type = commit_type;
    }

    pub(crate) fn set_commit_body(&mut self, pair: Pair<Rule>) {
        let body = pair.as_str();
        if !body.is_empty() {
            self.body = Some(body.to_string())
        }
    }

    pub(crate) fn set_footers(&mut self, pair: Pair<Rule>) {
        for footer in pair.into_inner() {
            self.set_footer(footer);
        }
    }

    fn set_footer(&mut self, footer: Pair<Rule>) {
        let footer = Footer::from(footer);

        if footer.is_breaking_change() {
            self.is_breaking_change = true;
        }

        self.footers.push(footer);
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
            "refactor" => Refactor,
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

impl AsRef<str> for CommitType {
    fn as_ref(&self) -> &str {
        match self {
            Feature => "feat",
            BugFix => "fix",
            Chore => "chore",
            Revert => "revert",
            Performances => "perf",
            Documentation => "docs",
            Style => "style",
            Refactor => "refactor",
            Test => "test",
            Build => "build",
            Ci => "ci",
            Custom(key) => key,
        }
    }
}

impl ToString for ConventionalCommit {
    fn to_string(&self) -> String {
        let mut message = String::new();
        message.push_str(self.commit_type.as_ref());

        if let Some(scope) = &self.scope {
            message.push_str(&format!("({})", scope));
        }

        let has_breaking_change_footer = self.footers.iter().any(|f| f.is_breaking_change());

        if self.is_breaking_change && !has_breaking_change_footer {
            message.push('!');
        }

        message.push_str(&format!(": {}", &self.summary));

        if let Some(body) = &self.body {
            message.push_str(&format!("\n\n{}", body));
        }

        if !self.footers.is_empty() {
            message.push('\n');
        }

        self.footers.iter().for_each(|footer| {
            message.push_str(&format!("\n{}: {}", footer.token, footer.content))
        });

        message
    }
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod test {
    use crate::commit::{CommitType, ConventionalCommit, Footer};
    use crate::parse;
    use indoc::indoc;
    use spectral::assert_that;
    use spectral::prelude::ResultAssertions;

    #[test]
    fn commit_to_string_ok() {
        let commit = ConventionalCommit {
            commit_type: CommitType::Feature,
            scope: None,
            summary: "a feature".to_string(),
            body: None,
            footers: Vec::with_capacity(0),
            is_breaking_change: false,
        };

        let expected = "feat: a feature".to_string();

        assert_that(&commit.to_string()).is_equal_to(expected);
        let parsed = parse(&commit.to_string());
        assert_that(&parsed).is_ok().is_equal_to(commit);
    }

    #[test]
    fn commit_to_with_footer_only_string_ok() {
        let commit = ConventionalCommit {
            commit_type: CommitType::Chore,
            scope: None,
            summary: "a commit".to_string(),
            body: None,
            footers: vec![Footer {
                token: "BREAKING CHANGE".to_string(),
                content: "message".to_string(),
            }],
            is_breaking_change: true,
        };

        let expected = indoc!(
            "chore: a commit

        BREAKING CHANGE: message"
        )
        .to_string();

        assert_that(&commit.to_string()).is_equal_to(expected);
        let parsed = parse(&commit.to_string());
        assert_that(&parsed).is_ok().is_equal_to(commit);
    }

    #[test]
    fn commit_with_body_only_and_breaking_change() {
        let commit = ConventionalCommit {
            commit_type: CommitType::Chore,
            scope: None,
            summary: "a commit".to_string(),
            body: Some("A breaking change body on\nmultiple lines".to_string()),
            footers: Vec::with_capacity(0),
            is_breaking_change: true,
        };

        let expected = indoc!(
            "chore!: a commit

            A breaking change body on
            multiple lines"
        )
        .to_string();

        assert_that(&commit.to_string()).is_equal_to(expected);
        let parsed = parse(&commit.to_string());
        assert_that(&parsed).is_ok().is_equal_to(commit);
    }

    #[test]
    fn full_commit_to_string() {
        let commit = ConventionalCommit {
            commit_type: CommitType::BugFix,
            scope: Some("code".to_string()),
            summary: "correct minor typos in code".to_string(),
            body: Some(
                indoc!(
                    "see the issue for details

        on typos fixed."
                )
                .to_string(),
            ),
            footers: vec![
                Footer {
                    token: "Reviewed-by".to_string(),
                    content: "Z".to_string(),
                },
                Footer {
                    token: "Refs".to_string(),
                    content: "133".to_string(),
                },
            ],
            is_breaking_change: false,
        };

        let expected = indoc!(
            "fix(code): correct minor typos in code

        see the issue for details

        on typos fixed.

        Reviewed-by: Z
        Refs: 133"
        )
        .to_string();

        assert_that(&commit.to_string()).is_equal_to(expected);
        let parsed = parse(&commit.to_string());

        assert_that(&parsed).is_ok().is_equal_to(commit);
    }
}
