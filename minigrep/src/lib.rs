
use std::{fs, error::Error};


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for `{}` in file `{}`", config.query, config.file_path);

    // read the ENTIRE contents of the file
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text:\n---BEGIN---\n{contents}\n---END---\n");

    Ok(())
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
