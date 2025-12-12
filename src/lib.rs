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
    pub invert_match: bool,

    #[arg(short='w', long="whole-word")]
    pub whole_word: bool
}

pub fn run(cfg : Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(cfg.file_path)?;

    // let results = search_demo(&config.query, &contents, config.ignore_case, config.invert_match, config.whole_word);
    let results = search_demo(&cfg.query, &contents, cfg.ignore_case, cfg.invert_match, cfg.whole_word);


    for line in results  {
        println!("{line}");
    }

    Ok(()) 
}

fn search_demo<'a>(query: &str, contents: &'a str, ignore_case: bool, invert_match: bool, whole_word: bool) -> Vec<&'a str> {
    let query: String = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    
    let results: Vec<&str> = contents
        .lines()
        .filter(|line: &&str|{
            let matches = matches_line(line, &query, ignore_case, whole_word);
            matches != invert_match
        })
        .collect();
    results
}

fn matches_line(line: &str, query: &str, ignore_case: bool, whole_word: bool) -> bool {
    // if whole then split on all non-alphanumeric instances and match
    if whole_word{
        let compare = |word: &str|{
            if ignore_case { word.to_lowercase() == query} else { word == query}
        };
        line.split(|c: char| !c.is_alphanumeric()).any(compare)
    }
    // else simple search
    else {
        let line_to_check = if ignore_case { line.to_lowercase()} else { line.to_string()};
        line_to_check.contains(query)
    }

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
        let query = "by";
        // don't insert a newline character 
        let contents = "\
Rust:
safe, fast, productive.
Duct three.
Lullaby
Made by me";
        
        assert_eq!(vec!["safe, fast, productive."], search_demo(query, contents, true, true, true));
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

