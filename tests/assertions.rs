#![allow(unused)]

use conventional_commit_parser::commit::{CommitType, ConventionalCommit, Footer};
use conventional_commit_parser::error::{ParseError, ParseErrorKind};
use speculoos::prelude::*;
use std::fmt::Debug;

pub fn assert_summary(res: &Result<ConventionalCommit, ParseError>, expected: &str) {
    assert_that(res)
        .is_ok()
        .map(|commit| &commit.summary)
        .is_equal_to(expected.to_string());
}

pub fn assert_commit_type(res: &Result<ConventionalCommit, ParseError>, expected: CommitType) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.commit_type)
        .is_equal_to(expected);
}

pub fn assert_no_scope(res: &Result<ConventionalCommit, ParseError>) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.scope)
        .is_none();
}

pub fn assert_scope(res: &Result<ConventionalCommit, ParseError>, expected: &str) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.scope)
        .is_some()
        .is_equal_to(expected.to_string());
}

pub fn assert_breaking_change(res: &Result<ConventionalCommit, ParseError>) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.is_breaking_change)
        .is_true();
}

pub fn assert_not_breaking_change(res: &Result<ConventionalCommit, ParseError>) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.is_breaking_change)
        .is_false();
}

pub fn assert_no_body(res: &Result<ConventionalCommit, ParseError>) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.body)
        .is_none()
}

pub fn assert_body(res: &Result<ConventionalCommit, ParseError>, expected: &str) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.body)
        .is_some()
        .is_equal_to(expected.to_string());
}

pub fn assert_no_footers(res: &Result<ConventionalCommit, ParseError>) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.footers)
        .is_empty()
}

pub fn assert_contains_footer(res: &Result<ConventionalCommit, ParseError>, expected: Footer) {
    assert_that(res)
        .is_ok()
        .map(|message| &message.footers)
        .contains(expected)
}

pub fn assert_error<T: Debug>(res: &Result<T, ParseError>, expected: ParseErrorKind) {
    assert_that(res)
        .is_err()
        .map(|err| &err.kind)
        .is_equal_to(expected)
}
