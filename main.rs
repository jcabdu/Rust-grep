use std::env;           // to read arguments from the prompt - 
use std::process; 

use minigrep_0_2::Config; 

fn main() {
    let args: Vec <String> = env::args().collect();         // env::args() function returns an iterator - .collect() into a Vec - 

    let config= Config::new (&args).unwrap_or_else(|err| {
        eprintln! ("Problem parsing arguments: {}", err); 
        process::exit(1); 
    }); 

    println! ("Searching for the query: {}", config.query); 
    println! ("In the file: {}", config.filename); 

    if let Err(e)= minigrep_0_2::run (config) {
        eprintln! ("Application error: {}", e); 
        process::exit(1); 
    }
}
