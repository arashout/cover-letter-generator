use std::collections::HashSet;
use std::iter::FromIterator;

extern crate regex;
use regex::Regex;

lazy_static!{
    static ref RE: Regex = Regex::new(r"[',!$%^&*;:{}=_`~().-]").expect("Could not compile regex");
}
lazy_static!{
    static ref RE_SPLIT: Regex = Regex::new(r"/").expect("Could not compile regex");
}

pub fn hashset<'a>(data: &Vec<&'a str>) -> HashSet<String> {
    let mut hs = HashSet::new();
    for s in data {
        hs.insert((*s).to_owned());
    }
    return hs;
}

// TODO: Is it possible to extend Vec methods?
fn contains<T>(v: &Vec<T>, needle: &T) -> bool
where
    T: PartialEq,
{
    v.iter().fold(false, |acc, item| acc || item == needle)
}

pub fn tokenize(text: &str) -> HashSet<String> {
    // let text = earth::sanitizer::clean_html(text);
    let text = RE
        .replace_all(text, "");
    let text = RE_SPLIT
        .replace_all(&text, " ")
        .to_ascii_lowercase();

    let tokens =
        text
        .split_whitespace()
        .collect();
    hashset(&tokens)
}

// TODO: Figure out what's going on here
//  let tokens: Vec<&str> = RE
//    |  _____________________________^
// 22 | |         .replace_all(text, "")
// 23 | |         .to_ascii_lowercase()
//    | |_____________________________^ creates a temporary which is freed while still in use
// 24 |           .split_whitespace()
// 25 |           .collect();
//    |                     - temporary value is freed at the end of this statement
// 26 |       hashset(&tokens)
//    |               ------- borrow later used here
