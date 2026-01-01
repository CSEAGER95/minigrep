use std::error::Error;
use minigrep::search;
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
    // dbg!(args);
                //printing a vector using a debug macro
                //the first value in the args vector will be
                //"target/debug/minigrep", which is name of the binary or the "program name"
    



    print!("searching for {}", config.query);
    println!(" in file {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { //dyn is short for dynamic, imported with
                                                       //error at the top. this allows us to return
                                                       //a type that implements the error trait
    let contents = fs::read_to_string(config.file_path)?; //? returns the error value from the
                                                          //current functions
    for line in search(&config.query, &contents) {
        println!("{}",line);
    }

    Ok(())// successful return value is an Ok type
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); //cloning data is an inefficcient way to handle ownership
                                     //problems. 
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}
