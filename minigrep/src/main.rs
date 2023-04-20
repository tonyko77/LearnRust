//---------------------
//     Mini-grep
// A tutorial project
//---------------------

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // check arguments and print usage if not ok
    if args.len() < 3 {
        let cmd = &args[0];
        let mut idx = cmd.rfind('/');
        if idx.is_none() {
            idx = cmd.rfind('\\');
        }
        let cmd = 
            if let Some(i) = idx {
                &cmd.as_str()[(i+1)..]
            }
            else {
                cmd.as_str()
            };
        println!("Usage: {cmd} <query string> <path/to/file>");
        return;
    }

    let config = parse_config(&args);
    println!("Searching for `{}` in file `{}`", config.query, config.file_path);

    // read the ENTIRE contents of the file
    let contents = fs::read_to_string(&config.file_path)
        .expect(format!("Could not read the file {}", config.file_path).as_str());

    println!("With text:\n{contents}\n");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
