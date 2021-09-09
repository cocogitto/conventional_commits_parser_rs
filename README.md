# Conventional commit parser
![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/oknozor/conventional_commits_parser_rs)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/oknozor/conventional_commits_parser_rs)


A rust implementation of the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/).
---

## Example 

```rust
use conventional_commit_parser::parse;
use conventional_commit_parser::commit::*;
let message = r#"fix: correct minor typos in code

see the issue for details

on typos fixed.

Reviewed-by: Z
Refs #133"#;

let conventional_commit = parse(message)?;

assert_eq!(conventional_commit.commit_type, CommitType::BugFix);
assert_eq!(conventional_commit.summary, "correct minor typos in code".to_string());
assert_eq!(conventional_commit.body, Some(r#"see the issue for details

on typos fixed."#.to_string()));

assert_eq!(conventional_commit.footers, vec![
    Footer {token: "Reviewed-by".to_string(), content: "Z".to_string()},
    Footer {token: "Refs".to_string(), content: "133".to_string(),}
]);
```

## Licence

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.
