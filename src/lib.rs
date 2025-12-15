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
    pub whole_word: bool,

    #[arg(short='n', long="line-numbers")]
    pub line_numbers: bool
}

pub fn run(cfg : Config) -> Result<(), Box<dyn Error>>{
    let contents: String = fs::read_to_string(&cfg.file_path)?;

    // let results = search_demo(&config.query, &contents, config.ignore_case, config.invert_match, config.whole_word);
    let results = search(&contents, &cfg);

    for (num, line_num) in results  {
        match cfg.line_numbers {
            true => println!("{}: {}", num + 1, line_num),
            false => println!("{}", line_num)
        }
    }


    Ok(()) 
}

fn search<'a>(contents: &'a str, cfg: &Config) -> Vec<(usize ,&'a str)> {
    let query: String = if cfg.ignore_case {
        cfg.query.to_lowercase()
    } else {
        cfg.query.to_string()
    };
    
    let results= contents
        .lines()
        .enumerate()
        .filter(|(_number,line)|{
            let matches: bool = matches_line(line, &query, cfg.ignore_case, cfg.whole_word);
            matches != cfg.invert_match // exploit the inverse boolean relationship between the two
        })
        .collect();

    results
}

fn matches_line(line: &str, query: &str, ignore_case: bool, whole_word: bool) -> bool {
    // if whole then split on all non-alphanumeric instances and match 
    if whole_word{
        line.split(|c: char| !c.is_alphanumeric()).any(
            |word| if ignore_case {word.to_lowercase() == query} else {word == query}
        )
    }
    // else simple search
    else {
        let line_to_check = if ignore_case { line.to_lowercase()} else { line.to_string()};
        line_to_check.contains(query)
    }

}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
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
    fn case_insens_whole() {
        let query = "by";
        let filepath = "test.txt";
        let contents = "\
Rust:
safe, fast, productive.
Lullaby
Made by me
By us";

        let config: Config = Config { 
            query: query.to_string(), 
            file_path: filepath.to_string() , 
            ignore_case: true, 
            invert_match: false, 
            whole_word: true,
            line_numbers: false
        };
        // vec!["Rust:","safe, fast, productive.", "Lullaby", "Made by me", "By us"];
        assert_eq!(vec![(3, "Made by me"), (4,"By us")], search(&contents, &config));
    }

    #[test]
    fn case_insens(){
        let query = "by";
        let filepath = "test.txt";
        let contents = "\
Rust:
safe, fast, productive.
Lullaby
Made by me
By us";

        let config: Config = Config { 
            query: query.to_string(), 
            file_path: filepath.to_string() , 
            ignore_case: true, 
            invert_match: false, 
            whole_word: false,
            line_numbers: true
        };

        assert_eq!(
            vec![(2,"Lullaby"), (3,"Made by me"), (4,"By us")],
            search(&contents, &config)
        )
    }
}

