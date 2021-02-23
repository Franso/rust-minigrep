use std::env;
use std::error::Error;
use std::fs;

// The Config struct  will hold the two arguments
// This glues together our data into a reusable obj rather than going the primitive obsession way
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // create clones to borrow from args
        // the clones will be less efficient on runtime
        // if args.len() < 3 {
        //     // can return a panic here, but this is good for programming purposes
        //     // returnng a result type is better than panic! in usage
        //     // panic!("not enough arguments");
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// return type is the unit type () o in the case of an error the trait obect Box<dyn Error>
// Box<dyn Error> means that the function will return a type that implements the Error trait
// dynamic means we dont know the type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current function for the caller to handle
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    // println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    // function that takes query and searches from file returning just those lines
    use super::*;
    // #[test]
    // fn one_result() {
    //     let query = "duct";
    //     let contents = "
    //     Rust:\nsafe, fast, productive.\nPick three.";
    //     assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    // }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
        Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
        Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

//  we indicate using 'a that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // refactor and enhance
    // usng the filter adaptor
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // // Iterate through each line of the contents.
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     // Check whether the line contains our query string.
    //     // use the string
    //     if line.contains(query) {
    //         // If it does, add it to the list of values we’re returning.
    //         // If it doesn’t, do nothing.
    //         results.push(line);
    //     }
    // }
    // // Return the list of results that match.
    // results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
