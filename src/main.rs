// 1. Read the argument values
// To do this we have to use the standard library std::env::arg
// This will enable us to iterate over the command line arguments
// We then collect all the values into a vector using  the collect method

use std::env;
use std::fs;

fn main() {
    //collect cli arguments into a vector named args
    let args: Vec<String> = env::args().collect();

    // Saving the argument values in variables
    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        // create clones to borrow from args
        // the clones will be less efficient on runtime
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
