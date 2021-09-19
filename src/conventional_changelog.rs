use crate::conventional_changelog::Rule as ChangelogRule;
use pest::error::Error as PestError;
use pest::Parser;

#[doc(hidden)]
#[derive(Parser)]
#[grammar = "conventional_changelog_grammar.pest"]
struct ConventionalChangelogConfigParser;

#[derive(Debug)]
pub enum Token {
    Host,
    Repository,
    Owner,
    User,
    IssueId,
    Hash,
    CurrentTag,
    PreviousTag,
    Slash,
    Other(String),
}

pub fn parse(url_format: &str) -> Result<Vec<Token>, PestError<ChangelogRule>> {
    let pairs = ConventionalChangelogConfigParser::parse(ChangelogRule::url, url_format)?
        .next()
        .unwrap();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            ChangelogRule::host => println!("host : {}", pair.as_str()),
            ChangelogRule::slash => println!("slash"),
            ChangelogRule::other => println!("other : {}", pair.as_str()),
            ChangelogRule::substitution => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        ChangelogRule::owner => println!(" owner : {}", pair.as_str()),
                        ChangelogRule::repository => println!(" repo : {}", pair.as_str()),
                        ChangelogRule::hash => println!("hash : {}", pair.as_str()),
                        ChangelogRule::previous_tag => println!("{}", pair.as_str()),
                        ChangelogRule::current_tag => println!("current tag : {}", pair.as_str()),
                        ChangelogRule::issue_id => println!(" issue id : {}", pair.as_str()),
                        ChangelogRule::user => println!("user : {}", pair.as_str()),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }


    Ok(vec![])
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("Default issue url");
        let vec = crate::conventional_changelog::parse("{{host}}/{{owner}}/{{repository}}/issues/{{id}}")
            .unwrap();

        println!("Default user url");
        let vec = crate::conventional_changelog::parse("{{host}}/{{user}")
            .unwrap();

        println!("Default compare url");
        let vec = crate::conventional_changelog::parse("{{host}}/{{owner}}/{{repository}}/compare/{{previous_tag}}...{{current_tag}}")
            .unwrap();

        println!("Default release commit");
        let vec = crate::conventional_changelog::parse("chore(version): {{version}}")
            .unwrap();



        assert!(false);
    }
}

