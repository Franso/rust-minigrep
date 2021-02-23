// 1. Read the argument values
// To do this we have to use the standard library std::env::arg
// This will enable us to iterate over the command line arguments
// We then collect all the values into a vector using  the collect method

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //collect cli arguments into a vector named args
    // let args: Vec<String> = env::args().collect();

    // Creating a new instance of Config
    // in case we have less than two args the err from the new module in Config raises an error
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // this anonymous function is called a closure
        // unwrap_or_else creates a custom non-panic error handling
        // use eprintln! so that the messages are printed to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        // process::exit will stop the program immediately and return the number that was passed as the exit status code
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // 2. Read from the file entered
    // handle any errors that might come from the file reading (run)function
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
