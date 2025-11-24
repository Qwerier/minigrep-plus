use clap::Parser;
use repgrep::Config;

use std::process;

fn main() {
    let config = Config::try_parse()
      .unwrap_or_else(|err|{
        eprintln!("{err}");
        process::exit(1);
      });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.file_path);

    match repgrep::run(config) {
      Err(err)=> {
        eprintln!("Application error: {err}");
        process::exit(1);
      },
      _ => {},
    };
}