use std::error::Error;
use std::{ fs};
use clap::{Parser};

#[derive(Debug, Parser)]
pub struct Config {
    pub query: String,
    pub file_path: String,

    #[arg(short= 'i', long= "ignore-case")]
    pub ignore_case: bool,

    #[arg(short= 'v', long= "invert-match")]
    pub invert_match: bool
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results  {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| {
            let token = query.to_lowercase();
            line.to_lowercase().contains(&token.to_lowercase())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // don't insert a newline character 
        let contents = "\
Rust:
safe, fast, productive.
Duct three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

