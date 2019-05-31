use std::collections::HashSet;

use crate::types::TokenizedDescription;
use crate::types::Blurb;

pub type Rule = Box<Fn(&TokenizedDescription) -> bool>;

pub fn any_in(tokens: std::vec::Vec<&'static str>) -> Rule {
    let c = move |tokenized_description: &TokenizedDescription| -> bool {
        for token in tokens.iter() {
            if tokenized_description.contains(*token) {
                return true;
            }
        }
        false
    };
    Box::new(c)
}

pub fn all_in(tokens: std::vec::Vec<&'static str>) -> Rule {
    let c = move |tokenized_description: &TokenizedDescription| -> bool {
        for token in tokens.iter() {
            if !tokenized_description.contains(*token) {
                return false;
            }
        }
        true
    };
    Box::new(c)
}

pub fn apply_rules(tokenized_description: &TokenizedDescription, blurb: Blurb) -> bool {
    unimplemented!()
}
// TODO: add tests for rules
