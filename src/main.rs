use std::collections::HashMap;
use std::env;
use std::io::{stdin, Read};

mod types;
use crate::types::{BlurbVec, KeywordBlurb, KeywordGroup};

mod config;
use crate::config::{BLURBS, KEYWORDS_GROUP_MAP};

mod utils;
use crate::utils::{hashset, tokenize};

// Used in config, utils
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

extern crate earth;

fn calculate_job_score(description: &String, keywords_groups: &HashMap<&str, KeywordGroup>) -> i32 {
    let tokenized_description = tokenize(description);
    // TODO: Why doesn't this work???
    // let tokenized_description: Vec<&str> = description.to_lowercase().split_whitespace().collect();
    let mut score = 0;
    for (_, keyword_group) in keywords_groups.iter() {
        // TODO: Make any_contains helper
        for trigger_token in keyword_group.trigger_tokens.iter() {
            // Check if the keyword_token is equal to any of the tokens from the description
            if tokenized_description.contains(*trigger_token) {
                score += keyword_group.score;
                break;
            }
        }
    }
    score
}

fn generate_message<'a>(description: &String, blurbs: &BlurbVec<'a>) -> String {
    let mut message = String::new();
    message.push_str(&format!(
        "{}\n\n",
        blurbs
            .get(0)
            .expect("Expected intro blurb")
            .long_description
    ));
    let tokenized_description = tokenize(description);
    print!("{:?}", tokenized_description);
    for i in 1..blurbs.len() - 1 {
        let blurb: &KeywordBlurb = blurbs
            .get(i)
            .expect(&format!("Expected blurb at index: {}", i));
        for trigger_token in blurb.trigger_tokens.iter() {
            if tokenized_description.contains(*trigger_token) {
                message.push_str(&format!("-{}\n", blurb.long_description));
                break;
            }
        }
    }
    message.push_str(&format!(
        "\n{}",
        blurbs
            .get(blurbs.len()-1)
            .expect("Expected outro blurb")
            .long_description
    ));
    message
}

// NOTE: cfg is telling it whether to compile or not
#[cfg(test)]
mod test {
    use super::{calculate_job_score, KeywordGroup};

    #[test]
    fn test_job_score() {
        let score = calculate_job_score(
            &"javascripts typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["react"]}],
        );
        assert!(
            score == 0,
            "Score should be zero since no tokens in description"
        );
        let score = calculate_job_score(
            &"javascript typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
        );
        assert!(
            score == 5,
            "Score should be 5 since atleast one of the tokens from same keygroup found"
        );
        let score = calculate_job_score(
            &"javascripts typescripts react".to_owned(),
            &hashmap![
                "frontend" => KeywordGroup{ score: 3, trigger_tokens: vec!["react"]},
                "backend" => KeywordGroup{ score: 5, trigger_tokens: vec!["java"]},
            ],
        );
        assert!(score == 3, "Score should be 3 since java !== javascript");
        let score = calculate_job_score(
            &"javascripts typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
        );
        assert!(
            score == 0,
            "Score should be 0 since no stemming" // TODO: Should i be stemming "javascripts" to "javascript"
        );
    }
}

fn main() {
    // Get command-line arguments to determine which mode to operate in
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("message-generator -m  == generate message for description\nmessage-generator -s == score the job description for suitablity");
        std::process::exit(-1);
    }
    // Get description from stdin used to either score or generate a message
    let mut description = String::new();
    stdin()
        .read_to_string(&mut description)
        .expect("Could not read from stdin");

    match args.get(1) {
        None => {
            println!("No argument provided!");
        }
        Some(mode) => {
            if *mode == "-m".to_owned() {
                let message = generate_message(&description, &BLURBS);
                println!("{}", message);
            } else if *mode == "-s".to_owned() {
                let score = calculate_job_score(&description, &KEYWORDS_GROUP_MAP);
                println!("{}", score);
            }
        }
    }
}
