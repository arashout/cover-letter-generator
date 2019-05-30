# cv-generator
A command-line application for generating messages, determining fit and picking a correct resume to send based off the job description.

# TODO:
- Use a rules engine to implement rules for determining when to insert blurbs (Might be worth looking into how clap does this):
    - all (Right not trigger tokens are any)
    - requires
    - conflicts_with 
- Start implementing cutoff for messages, have a short and long version for blurbs
- Use builder pattern to construct types: https://users.rust-lang.org/t/idiomatic-way-to-construct-object-with-some-non-required-fields/8078/5
- Use config to pass arguments to message generator function instead of having to pass a ton of arguments