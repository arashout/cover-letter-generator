#[derive(Debug)]
pub struct KeywordGroup<'a> {
    pub score: i32,
    pub trigger_tokens: Vec<&'a str>,
}
#[derive(Debug, Default)]
pub struct KeywordBlurb<'a> {
    pub precendence: u8,
    pub long_description: &'a str,
    pub trigger_tokens: Vec<&'a str>,
    pub name: &'a str,
}

// TODO: Eventually move to graphs once dependencies are figured out
pub type BlurbVec<'a> = Vec<KeywordBlurb<'a>>;