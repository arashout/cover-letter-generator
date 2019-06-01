use crate::rules::{Rule, apply_rules};
use crate::types::TokenizedDescription;

#[derive(Default)]
pub struct Blurb<'a> {
    pub precendence: u8,
    pub long_description: &'a str,
    pub short_description: &'a str,
    pub name: &'a str,
    rules: Vec<Box<Rule>>,
}

impl<'a> Blurb<'a> {
    pub fn new(name: &'a str) -> Self {
        Blurb {
            name: name,
            precendence: 10,
            long_description: &"",
            short_description: &"",
            rules: vec![],
        }
    }

    pub fn with_precedence(mut self, n: u8) -> Self {
        self.precendence = n;
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
    pub fn add_rule(mut self, boxed_rule: Box<Rule>) -> Self {
        self.rules.push(boxed_rule);
        self
    }

    pub fn is_applicable(&self, tokenized_description: &TokenizedDescription) -> bool {
        if self.rules.len() == 0 {
            return false;
        }
        apply_rules(tokenized_description, &self.rules)
    }
}
pub type BlurbVec<'a> = Vec<Blurb<'a>>;
