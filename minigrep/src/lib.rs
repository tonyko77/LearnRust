//--------------------------------
// Minigrep - main functionality
//--------------------------------

use std::{fs, env, error::Error};


//-------------------
//  Program Config

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
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
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            
            Ok(Self {
                query,
                file_path,
                ignore_case
            })
        }
    }
}


//-----------------------
//  Main Functionality

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    }
    else {
        search_case_sensitive(config.query.as_str(), contents.as_str())
    };

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


pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
        let results = search_case_sensitive(query, TEST_CONTENTS);
        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn no_result() {
        let query = "x";
        let results = search_case_sensitive(query, TEST_CONTENTS);
        assert_eq!(Vec::<&str>::new(), results);
    }

    const CS_TEST_CONTENTS: &'static str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct tape.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let results = search_case_sensitive(query, CS_TEST_CONTENTS);
        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let results = search_case_insensitive(query, CS_TEST_CONTENTS);
        assert_eq!(vec!["Rust:", "Trust me."], results);
    }

}
