# Conventional commit parser
[![Latest Version]][crates.io]
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
![License](https://img.shields.io/github/license/oknozor/conventional_commits_parser_rs)

[Latest Version]: https://img.shields.io/crates/v/conventional_commit_parser.svg
[crates.io]: https://www.crates.io/crates/conventional_commit_parser

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

let commit = parse(message).unwrap();

assert_eq!(commit.commit_type, CommitType::BugFix);
assert_eq!(commit.summary, "correct minor typos in code".to_string());
assert_eq!(commit.body, Some(r#"see the issue for details

on typos fixed."#));

assert_eq!(commit.footers, vec![
    Footer {token: "Reviewed-by", content: "Z"},
    Footer {token: "Refs", content: "133",}
]);
```

## Licence

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.
