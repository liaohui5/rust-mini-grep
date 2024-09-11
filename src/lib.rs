//! # minigrep_4_study_rust crate
//!
//! `minigrep_4_study_rust create` is a collection cli arguments utilities

use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
   /// Create config instance
   /// ### Example
   /// ```rust
   /// use minigrep_4_study_rust::Config;
   /// let args = std::env::args();
   /// let config = Config::new(args);
   /// ```
   pub fn new(mut iter: std::env::Args) -> Result<Config, &'static str> {
        iter.next();

        let query = match iter.next() {
            None => return Err("No query string"),
            Some(query) => query
        };

        let filename = match iter.next() {
            None => return Err("No filename string"),
            Some(filename) => filename
        };

        let mut config = Self {
           query,
           filename,
           ignore_case: false,
        };

        for item in iter {
            if item == "-I" || item == "--ignore-case" {
                config.ignore_case = true;
                break;
            }
        }

       Ok(config)
    }
}

pub fn run (config: Config)-> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    let matched = if config.ignore_case {
        search_ignore_case(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for item in matched {
        println!("{}", item);
    }

    Ok(())
}

/// Search query in file contents
///
/// # Search string in file contents
///
/// ```
/// use minigrep_4_study_rust::search;
/// let matches = search("hello", "other string hello other string");
/// let expected = vec!["other string hello other string"];
/// assert_eq!(expected, matches);
/// ```
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_ignore_case<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    content.lines().filter(|line| {
        line.to_lowercase().contains(&query.to_lowercase())
    }).collect()
}

#[cfg(test)]
mod unit_tests {
    use crate::*;

    #[test]
    fn should_be_match() {
        let contents = "\
other lines
hello world
HELLO WORLD
other lines
";
        let expected = vec!["hello world"];
        assert_eq!(expected, search("hello", contents));
    }


    #[test]
    fn should_be_ignore_case_match() {
        let contents = "\
other lines
hello world
HELLO WORLD
other lines
";
        let expected = vec!["hello world", "HELLO WORLD"];
        assert_eq!(expected, search_ignore_case("hello", contents));
    }
}
