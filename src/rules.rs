use crate::types::TokenizedDescription;

pub type Rule = Fn(&TokenizedDescription) -> bool;

pub fn any_in(tokens: std::vec::Vec<&'static str>) -> Box<Rule> {
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

pub fn all_in(tokens: Vec<&'static str>) -> Box<Rule> {
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

pub fn apply_rule(tokenized_description: &TokenizedDescription, rule: &Rule) -> bool {
    (*rule)(tokenized_description)
}

pub fn apply_rules(tokenized_description: &TokenizedDescription, rules: &[Box<Rule>]) -> bool {
    for boxed_rule in rules.iter(){
        let rule = &**boxed_rule;
        if !apply_rule(tokenized_description, rule) {
            return false;
        }
    }
    true
}

// NOTE: cfg is telling it whether to compile or not
#[cfg(test)]
mod test {
    use super::{apply_rules, all_in, apply_rule};
    use crate::utils::{tokenize};

    #[test]
    fn test_all_in() {
       let tokenized_description = tokenize("you must love typescript and rust");
       let rule1 = all_in(vec!["typescript", "rust"]);
       assert!(apply_rule(&tokenized_description, &*rule1), "Applying single rule incorrect");

       let rule2 = all_in(vec!["javascript"]);
       let rules = vec![rule1, rule2];
       assert!(apply_rules(&tokenized_description, &rules) == false, "Applying multiple rules incorrect");
    }
}