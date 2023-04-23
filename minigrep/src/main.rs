//---------------------
//     Mini-grep
// A tutorial project
//---------------------

use minigrep::Config;

use std::{env, process};


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        let cmd = minigrep::extract_program_name(&args);
        eprintln!("Usage: {cmd} <query string> <path/to/file>");
        process::exit(1);
    });

    let result = minigrep::run(&config);
    if let Err(e) = result {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
