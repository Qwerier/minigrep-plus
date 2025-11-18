use repgrep::Config;

use std::{env, process};

fn main() {
    let args:env::Args = env::args();

    let config = Config::build(args)
        .unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {err}");
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