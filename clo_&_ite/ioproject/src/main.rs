use std::env;
use std::process;

use ioproject::Config;

fn main() {
    // --snip--
    //let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file: {}", config.file_path);
    println!("##############################");

    if let Err(e) = ioproject::run(config) {
        // --snip--
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}