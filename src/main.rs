// 1. Read the argument values
// To do this we have to use the standard library std::env::arg
// This will enable us to iterate over the command line arguments
// We then collect all the values into a vector using  the collect method
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //collect cli arguments into a vector named args
    println!("{:?}", args);
}
