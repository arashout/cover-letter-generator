use std::collections::HashSet;

use crate::rules::Rule;

pub type TokenizedDescription = HashSet<String>;

#[derive(Debug)]
pub struct KeywordGroup<'a> {
    pub score: i32,
    pub trigger_tokens: Vec<&'a str>,
}
#[derive(Debug, Default)]
pub struct Blurb<'a> {
    pub precendence: u8,
    pub long_description: &'a str,
    pub short_description: &'a str,
    pub trigger_tokens: Vec<&'a str>,
    pub name: &'a str,
    pub rules: Vec<Rule>
}

impl<'a> Blurb<'a> {
    pub fn new(name: &'a str) -> Self {
        Blurb {
            name: name,
            precendence: 10,
            long_description: &"",
            short_description: &"",
            trigger_tokens: vec![],
            rules: vec![]
        }
    }
    
    pub fn with_precedence(mut self, n: u8) -> Self {
        self.precendence = n;
        self
    }

    pub fn with_tokens(mut self, tokens: Vec<&'a str>) -> Self {
        self.trigger_tokens = tokens;
        self
    }

    pub fn with_description(mut self, description: &'a str) -> Self {
        self.short_description = description;
        self.long_description = description;
        self
    }
    pub fn with_long_description(mut self, description: &'a str) -> Self {
        self.long_description = description;
        self
    }
    pub fn add_rule(mut self, rule: Rule){
        
    }

    // TODO: Add rules
}
pub type BlurbVec<'a> = Vec<Blurb<'a>>;

// TODO: Use this instead of passing arguments to message
pub struct Config {
    pub debug: bool,
    pub company: Option<String>,
    pub position: Option<String>
}