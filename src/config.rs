use crate::blurb::{Blurb, BlurbVec};
use crate::rules::{all_in, any_in};
use crate::types::KeywordGroup;
use std::collections::HashMap;

lazy_static! {
    pub static ref KEYWORDS_GROUP_MAP: HashMap<&'static str, KeywordGroup<'static>> = {
        let m = hashmap! {
        "frontend" => KeywordGroup{ score: 3, trigger_tokens: vec!["react", "reactjs", "angular", "angularjs", "vue"]},
        "favourite_languages" => KeywordGroup {score: 5, trigger_tokens: vec!["rust", "typescript"]},
        "dotnet" => KeywordGroup{ score: 4, trigger_tokens: vec!["c#", "net", "dotnet"]},
        "rust" => KeywordGroup{ score: 5, trigger_tokens: vec!["rust"]},
        "golang" => KeywordGroup{ score: 2, trigger_tokens: vec!["golang"]},
        "microservices" => KeywordGroup{ score: 3, trigger_tokens: vec!["microservices", "docker"]},
        "python" => KeywordGroup{ score: 5, trigger_tokens: vec!["python", "flask"]}
        };
        m
    };
}

pub fn get_blurbs() -> BlurbVec<'static> {
    let mut v = Vec::new();
    v.push(
        Blurb::new("intro")
            .with_precedence(255)
            .with_description("Hey I'm Ash,\
            \nA new grad from UBC with experience building APIs and client facing dashboard while at Ravelin.\
            \nI'm interested in the {POSITION} and below are a couple reasons I would be good fit:")
    );
    v.push(
        Blurb::new("typescript")
            .with_description("I have had the pleasure of building Ravelin's customer facing dashboard using AngularJS and TypeScript for year.
            TypeScript is one of my favorite languages as it has made working with large complex JavaScript applications a pleasure!")
            .add_rule(any_in(vec!["angular", "angularjs"]))
            .add_rule(all_in(vec!["typescript"]))
        );
    v.push(
        Blurb::new("react")
            .with_description("I built chrome extension for the sales team using React+TypeScript for prospecting clients that they \
            use everyday.")
            .with_long_description("I built chrome extension for the sales team using React+TypeScript for prospecting clients that they \
            use everyday. This project was a favor for a co-worker and purely my own initiative but it ended up being used by everyone on sales.\
            \nUsing the same tech stack I built my grandma a card matching game for learning the sounds of the English alphabet <3")
            .add_rule(any_in(vec!["react", "reactjs", "typescript"]))
            );
    v.push(
        Blurb::new("micro-go")
        .with_description("Familiar with the microservices archeticture and associated technologies like Docker since \
            at Ravelin built and tested new API endpoints and backend functionality with Go."
        )
        .add_rule(any_in(vec!["microservices", "golang"]))
        );
    v.push(
        Blurb::new("sdet")
        .with_description("I'm particularly obsessed with automation, so much so that I built a robot to apply to jobs for me using \
            Selenium. Thus I have a significant amount of experience automating web browsers which will come in handy for automated testing."
        )
        .add_rule(any_in(vec!["selenium", "cypress", "phantamjs", "scraping"]))
        );
    v.push(
        Blurb::new("rust")
        .with_description("Rust is one of my favourite languages, in fact this message was generated using a Rust console application I built! \
        \nI'm still learning so I attend a bi-weekly Rust meet-up to stay up to date.")
            .add_rule(any_in(vec!["rust"]))
    );
    v.push(
        Blurb::new("dotnet")
        .with_description("Using C# I've built simple games in Unity and Sudoku solver that uses constraint propagation.")
        .add_rule(any_in(vec!["c#", "dotnet", ".net"]))
    );
    v.push(
        Blurb::new("vis")
        .with_description("I worked extensively on the Ravelin Connect product, which involved building graph visualizations using \
            D3.js and implementing algorithms for efficient graph processing."
        )
        .add_rule(any_in(vec!["graph", "algorithms", "visualization"]))
    );
    v.push(
        Blurb::new("linux")
            .with_precedence(1)
            .with_description(
                "Ubuntu is my daily driver so proficiency in a linux environment is not a problem.",
            )
            .add_rule(any_in(vec!["linux", "unix"])),
    );
    v.push(
        Blurb::new("outro")
            .with_precedence(1)
            .with_description("This seems like an incredible opportunity so please do not hesitate to arrange an interview or ask any questions!"
            )
    );
    v
}
