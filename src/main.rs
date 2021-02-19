// 1. Read the argument values
// To do this we have to use the standard library std::env::arg
// This will enable us to iterate over the command line arguments
// We then collect all the values into a vector using  the collect method

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //collect cli arguments into a vector named args

    // Saving the argument values in variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // 2. Read from the file entered

    let contents = fs::read_to_string(filename).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
