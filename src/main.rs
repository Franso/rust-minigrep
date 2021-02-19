// 1. Read the argument values
// To do this we have to use the standard library std::env::arg
// This will enable us to iterate over the command line arguments
// We then collect all the values into a vector using  the collect method

use std::env;
use std::fs;
use std::process;

fn main() {
    //collect cli arguments into a vector named args
    let args: Vec<String> = env::args().collect();

    // Creating a new instance of Config
    // in case we have less than two args the err from the new module in Config raises an error
    let config = Config::new(&args).unwrap_or_else(|err| {
        // this anonymous function is called a closure
        // unwrap_or_else creates a custom non-panic error handling
        println!("Problem parsing arguments: {}", err);
        // process::exit will stop the program immediately and return the number that was passed as the exit status code
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 2. Read from the file entered

    let contents =
        fs::read_to_string(config.filename).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
// the struct Config will hold the two arguments
// This glues together our data into a reusable obj rather than going the primitive obsession way
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        // create clones to borrow from args
        // the clones will be less efficient on runtime
        if args.len() < 3 {
            // can return a panic here, but this is good for programming purposes
            // returnng a result type is better than panic! in usage
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
