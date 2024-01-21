use std::{fs, io::Error};

pub struct Configuration<'a> {
    pub filename: &'a str,
    pub query: &'a str,
}
pub struct Match<'a> {
    pub text: &'a str,
    pub location: u16,
}

impl<'a> Configuration<'a> {
    pub fn new(arguments: &'a [String]) -> Result<Self, &'a str> {
        if arguments.len() < 3 {
            return Err("Arguments cannot be less than 3");
        }
        let filename = &arguments[2];
        let query = &arguments[1];
        Ok(Configuration { filename, query })
    }
}

pub fn read_file(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
}

pub fn query<'a>(content: &'a str, query: &str) -> Vec<Match<'a>> {
    let mut location = 1;
    let mut matches: Vec<Match> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            let match_ = Match {
                text: line,
                location: location,
            };
            matches.push(match_);
        }
        location = location + 1;
    }
    matches
}
