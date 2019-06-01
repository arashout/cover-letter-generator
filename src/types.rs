use std::collections::HashSet;

use crate::rules::Rule;

pub type TokenizedDescription = HashSet<String>;

#[derive(Debug)]
pub struct KeywordGroup<'a> {
    pub score: i32,
    pub trigger_tokens: Vec<&'a str>,
}

pub struct Config<'a> {
    pub debug: bool,
    pub company: Option<&'a str>,
    pub position: Option<&'a str>
}