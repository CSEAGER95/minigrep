use std::error::Error;
use minigrep::{search, search_case_insensitive, search_exact_word};
use std::process;
use std::fs; //file reader crate
use std::env;//a crate that allows for the accepting of args variables input when a function is run
             //from cli


fn main() {
    let args: Vec<String> = env::args().collect(); // accepting the inputs as strings and saving
                                                   // them in a vector called args
    
    let config = Config::build(env:: args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments {}", err);
        process::exit(1);
    });
        if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub exact_word: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let exact_word = env::var("EXACT_WORD").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
            exact_word,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents) 
    } else if config.exact_word {
        search_exact_word(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}