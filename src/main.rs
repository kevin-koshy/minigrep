use std::{env,process};
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String>= env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(10);
    }


    fn run(config: Config)  -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        println!("With text:\n{}", contents);
        Ok(())
    }

    let file_path = "/Users/kevinkoshy/Documents/Development/Rust/RustBook/minigrep/poem.txt";
    println!("In file {}", file_path);


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
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}