use std::collections::HashMap;

use crate::types::{KeywordGroup, KeywordBlurb, BlurbVec};

lazy_static! {
    pub static ref KEYWORDS_GROUP_MAP: HashMap<&'static str, KeywordGroup<'static>> = {
        let m = hashmap! {
        "frontend" => KeywordGroup{ score: 3, trigger_tokens: vec!["react", "reactjs", "angular", "angularjs", "vue"]},
        "favourite_languages" => KeywordGroup {score: 5, trigger_tokens: vec!["rust", "typescript"]},
        "dotnet" => KeywordGroup{ score: 4, trigger_tokens: vec!["c#", "net", "dotnet"]},
        "rust" => KeywordGroup{ score: 5, trigger_tokens: vec!["rust"]},
        "golang" => KeywordGroup{ score: 2, trigger_tokens: vec!["golang"]},
        "microservices" => KeywordGroup{ score: 3, trigger_tokens: vec!["microservices"]},
        "python" => KeywordGroup{ score: 5, trigger_tokens: vec!["python", "flask"]}
        };
        m
    };
}

// TODO: Move this to yaml
lazy_static! {
    pub static ref BLURBS: BlurbVec<'static> = {
        let mut v = Vec::new();
        // TODO: Code for replacing company and position
        v.push(
            KeywordBlurb{
                name: "intro",
                precendence: 255,
                trigger_tokens: vec![],
                long_description: "Hey I'm Ash,\
                \nA new grad from UBC with experience working as a software engineer building APIs and client facing dashboards.\
                \nI would be super excited to work at {COMPANY} as a {POSITON}, here are a couple reasons why I think I would be a good fit:"
            });
        v.push(
            KeywordBlurb{
                name: "typescript",
                precendence: 9,
                trigger_tokens: vec!["angular", "angularjs", "typescript"],
                long_description: "I have had the pleasure of building Ravelin's customer facing dashboard using AngularJS and TypeScript for year.
                TypeScript is one of my favorite languages as it has made working with large complex JavaScript applications much more ergonomic!"
            }
        );
        v.push(
            KeywordBlurb{
                name: "react",
                precendence: 9,
                trigger_tokens: vec!["react", "reactjs", "typescript"],
                long_description:  "During my time at Ravelin, built chrome extension using React+TypeScript for prospecting clients that they \
                use everyday. This was a side project, that I did not expect to get much use but mainly did as a favor for co-worker, funny \
                how small things can be large value adds.\nUsing the same tech stack I built my grandma a card matching game for learning the \
                sounds of the English alphabet. Check it out: https://arashout.github.io/alphabet-match/"
            }
        );
        v.push(
            KeywordBlurb{
                name: "micro-go",
                precendence: 9,
                trigger_tokens: vec!["golang", "microservices"],
                long_description: "Familiar with the microservices archeticture and associated technologies like Docker since \
                at Ravelin built and tested new API endpoints and backend functionality with Go."
            }
        );
        v.push(
            KeywordBlurb{
                name: "sdet",
                precendence: 9,
                trigger_tokens: vec!["selenium","cypress", "phantamjs", "scraping"],
                long_description: "I'm particularly obsessed with automation, so much so that I built a robot to apply to jobs for me using \
                Selenium. Thus I have a significant amount of experience automating web browsers which will come in handy for automated testing."
            }
        );
        v.push(
            KeywordBlurb{
                name: "rust",
                precendence: 9,
                trigger_tokens: vec!["rust"],
                long_description: "I've recently gotten interested in Rust and regularly attend a Rust meet-up in Vancouver.
                In fact this message was generated by a small Rust console application!"
            }
        );
        v.push(
            KeywordBlurb{
                name: "vis",
                precendence: 9,
                trigger_tokens: vec!["visualization", "graph", "algorithms"],
                long_description: "I worked extensively on the Ravelin Connect product, which involved building graph visualizations using \
                D3.js and implementing algorithms for efficient graph processing."
            }
        );
        v.push(
            KeywordBlurb{
                name: "linux",
                precendence: 4,
                trigger_tokens: vec!["linux", "unix"],
                long_description: "I use Ubuntu as my daily driver so proficiency in a linux environment is not a problem."
            }
        );
        v.push(
            KeywordBlurb{
                name: "outro",
                precendence: 10,
                trigger_tokens: vec![],
                long_description: "This seems like an incredible opportunity so please do not hesitate to arrange an interview or ask any questions!"
            }
        );
        v
    };
}

