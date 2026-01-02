use std::error::Error;
use minigrep::{search, search_case_insensitive};
use std::process;
use std::fs; //file reader crate
use std::env;//a crate that allows for the accepting of args variables input when a function is run
             //from cli


fn main() {
    let args: Vec<String> = env::args().collect(); // accepting the inputs as strings and saving
                                                   // them in a vector called args
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments {}", err);
        process::exit(1);
    });
        if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); //cloning data is an inefficcient way to handle ownership
                                     //problems. 
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case,})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents) 
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}