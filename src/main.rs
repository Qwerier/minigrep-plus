use std::{env, fs};

fn main() {
    println!("Hello, world!");
    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should be able to read the file");

    println!("File contains:\n{contents}");

    dbg!(args);
}
