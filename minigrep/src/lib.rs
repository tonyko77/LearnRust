//--------------------------------
// Minigrep - main functionality
//--------------------------------

use std::{fs, error::Error};


//-------------------
//  Program Config

pub struct Config {
    query: String,
    file_path: String,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        }
        else {
            // for now we're using clone(), for simplicity
            // (but later we should try to use references or other "smart" idioms)
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok(Self { query, file_path })
        }
    }
}


//-----------------------
//  Main Functionality

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(config.query.as_str(), contents.as_str());

    if results.is_empty() {
        println!("`{}` was not found in file {}", config.query, config.file_path);
    }
    else {
        println!("`{}` was found {} times in file {}:",
            config.query, results.len(), config.file_path);
        for line in results {
            println!("-> {line}");
        }
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn extract_program_name<'a>(args: &'a [String]) -> &'a str {
    let cmd = &args[0];
    let mut idx = cmd.rfind('/');
    if idx.is_none() {
        idx = cmd.rfind('\\');
    }
    if let Some(i) = idx {
        &cmd.as_str()[(i+1)..]
    }
    else {
        cmd.as_str()
    }
}


//-------------------
//  TESTS

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CONTENTS: &'static str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn one_result() {
        let query = "duct";
        let results = search(query, TEST_CONTENTS);
        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn no_result() {
        let query = "x";
        let results = search(query, TEST_CONTENTS);
        assert_eq!(Vec::<&str>::new(), results);
    }
}
