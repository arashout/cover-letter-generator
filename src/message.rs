use crate::types::{Blurb, BlurbVec};
use crate::utils::{tokenize};

pub fn generate_message<'a>(description: &String, blurbs: &BlurbVec<'a>, company: &str, position: &str, debug_flag: bool) -> String {
    let mut message = String::new();
    message.push_str(&format!(
        "{}\n\n",
        blurbs
            .get(0)
            .expect("Expected intro blurb")
            .long_description
    ));
    let tokenized_description = tokenize(description);
    if debug_flag {
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

    message = message.replace("{COMPANY}", company);
    message = message.replace("{POSITON}", position);
    message
}