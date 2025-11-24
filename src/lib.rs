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

pub fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(config.file_path)?;

    let results = search_demo(&config.query, &contents, config.ignore_case, config.invert_match, config.whole_word);

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
            let pred_result: bool = if whole_word {
                if ignore_case {
                    line
                        .split(|c: char| !c.is_alphanumeric())
                        .any(|word| word.to_lowercase() == query)
                }
                else {
                    line
                        .split(|c: char| !c.is_alphanumeric())
                        .any(|word| word == query)
                }
            } 
            else {
                if ignore_case {
                    line.to_lowercase().contains(&query)
                }
                else {
                    line.contains(&query)
                }
            }; 
            
            if invert_match {
                !pred_result
            }
            else {
                pred_result
            }
            
        })
        .collect();
    results
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

