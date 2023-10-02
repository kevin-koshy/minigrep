use std::{env,process};
use minigrep::Config;

fn main() {
    let args: Vec<String>= env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(10);
    }

    // let file_path = "/Users/kevinkoshy/Documents/Development/Rust/RustBook/minigrep/poem.txt";
    // println!("In file {}", file_path);


}
