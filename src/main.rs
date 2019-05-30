use std::collections::HashMap;
use std::io::{stdin, Read};

mod types;
use crate::types::{KeywordGroup};

mod config;
use crate::config::{BLURBS, KEYWORDS_GROUP_MAP};

mod utils;
use crate::utils::{hashset, tokenize};

mod message;
use crate::message::generate_message;

// Used in config, utils
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

extern crate earth;

use clap::{App, Arg};

fn calculate_job_score(
    description: &String,
    keywords_groups: &HashMap<&str, KeywordGroup>,
    debug_flag: bool,
) -> i32 {
    let tokenized_description = tokenize(description);
    if debug_flag {
        println!("{:?}", tokenized_description);
    }
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


// NOTE: cfg is telling it whether to compile or not
#[cfg(test)]
mod test {
    use super::{calculate_job_score, KeywordGroup};

    #[test]
    fn test_job_score() {
        let score = calculate_job_score(
            &"javascripts typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["react"]}],
            false,
        );
        assert!(
            score == 0,
            "Score should be zero since no tokens in description"
        );
        let score = calculate_job_score(
            &"javascript typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
            false,
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
            false,
        );
        assert!(score == 3, "Score should be 3 since java !== javascript");
        let score = calculate_job_score(
            &"javascripts typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
            false,
        );
        assert!(
            score == 0,
            "Score should be 0 since no stemming" // TODO: Should i be stemming "javascripts" to "javascript"
        );
    }
}

fn main() {
    let matches = App::new("cv-generator")
        .version("1.0")
        .author("Arash Outadi <arash.out@gmail.com")
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .help(
                    "Generates a message from job description, company name and position name\n\
                     Usage:\n\
                     cv-generator -m -c=COMPANY -p=POSITON\n\
                     And the job description is passed through standard input",
                )
                .conflicts_with("score")
                .conflicts_with("resume")
                .requires("company")
                .requires("position")
                .takes_value(false),
        )
        .arg(Arg::with_name("score").short("s").long("score").help(
            "Calculates a score based off of the job description\n\
             Usage:\n\
             cv-generator -s\n\
             And the job description is passed through standard input",
        ))
        .arg(Arg::with_name("resume").short("r").long("resume").help(
            "Recommends a resume based off the job description\n\
             Usage:\n\
             cv-generator -r \n\
             And the job description is passed through standard input",
        ))
        .arg(Arg::with_name("company").short("c").long("company").help("the company name").takes_value(true))
        .arg(Arg::with_name("position").short("p").long("position").help("the position name").takes_value(true))
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Extra messages printed out for debugging"),
        )
        .get_matches();

    // Get description from stdin used to either score or generate a message
    let mut description = String::new();
    stdin()
        .read_to_string(&mut description)
        .expect("Could not read from stdin");

    let debug_flag = matches.is_present("debug");
    if matches.is_present("message") {
        let message = generate_message(&description, &BLURBS, matches.value_of("company").unwrap(), matches.value_of("position").unwrap(), debug_flag);
        println!("{}", message);
    } else if matches.is_present("score") {
        let score = calculate_job_score(&description, &KEYWORDS_GROUP_MAP, debug_flag);
        println!("{}", score);
    } else if matches.is_present("resume") {
        unimplemented!();
    }
}
