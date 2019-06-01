use crate::types::{Blurb, Config};
use crate::utils::{tokenize};
use crate::config::{get_blurbs};

pub fn generate_message(description: &String, config: &Config) -> String {
    let blurbs = get_blurbs();
    let mut message = String::new();
    message.push_str(&format!(
        "{}\n\n",
        blurbs
            .get(0)
            .expect("Expected intro blurb")
            .long_description
    ));
    let tokenized_description = tokenize(description);
    if config.debug {
        println!("{:?}", tokenized_description);
    }
    for i in 1..blurbs.len() - 1 {
        let blurb: &Blurb = blurbs
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
            .get(blurbs.len() - 1)
            .expect("Expected outro blurb")
            .long_description
    ));

    message = message.replace("{COMPANY}", config.company.expect("No company provided"));
    message = message.replace("{POSITON}", config.position.expect("No position provided"));
    message
}