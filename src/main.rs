use std::collections::HashMap;
use std::io::{stdin, Read};

mod types;
use crate::types::{KeywordGroup, Config};

mod config;
use crate::config::{KEYWORDS_GROUP_MAP};

mod utils;
use crate::utils::{tokenize};

mod message;
use crate::message::generate_message;

mod blurb;

mod rules;

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
    config: &Config,
) -> i32 {
    let tokenized_description = tokenize(description);
    if config.debug {
        println!("{:?}", tokenized_description);
    }
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
    use super::{calculate_job_score, KeywordGroup, Config};

    #[test]
    fn test_job_score() {
        let config = Config{
            debug: false,
            company: None,
            position: None,
        };
        let score = calculate_job_score(
            &"javascripts typescript".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["react"]}],
            &config,
        );
        assert!(
            score == 0,
            "Score should be zero since no tokens in description"
        );
        let score = calculate_job_score(
            &"javascript typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
            &config,
        );
        assert!(
            score == 5,
            "Score should be 5 since atleast one of the tokens from same keygroup found"
        );
        let score = calculate_job_score(
            &"javascripts typescript react".to_owned(),
            &hashmap![
                "frontend" => KeywordGroup{ score: 3, trigger_tokens: vec!["react"]},
                "backend" => KeywordGroup{ score: 5, trigger_tokens: vec!["java"]},
            ],
            &config,
        );
        assert!(score == 3, "Score should be 3 since java !== javascript");
        let score = calculate_job_score(
            &"javascripts typescripts".to_owned(),
            &hashmap!["frontend" => KeywordGroup{ score: 5, trigger_tokens: vec!["javascript", "typescript"]}],
            &config,
        );
        assert!(
            score == 0,
            "Score should be 0 since no stemming"
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

    let config = Config{
        debug: matches.is_present("debug"),
        company: matches.value_of("company"),
        position: matches.value_of("position")
    };

    if matches.is_present("message") {
        let message = generate_message(&description, &config);
        println!("{}", message);
    } else if matches.is_present("score") {
        let score = calculate_job_score(&description, &KEYWORDS_GROUP_MAP, &config);
        println!("{}", score);
    } else if matches.is_present("resume") {
        unimplemented!();
    }
}
