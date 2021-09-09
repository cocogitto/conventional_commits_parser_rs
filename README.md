# Conventional commit parser

A rust implementation of the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/).


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
