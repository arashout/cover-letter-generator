use crate::blurb::Blurb;
use crate::config::get_blurbs;
use crate::types::Config;
use crate::utils::tokenize;

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

        if blurb.is_applicable(&tokenized_description) {
            message.push_str(&format!("-{}\n", blurb.long_description));
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
