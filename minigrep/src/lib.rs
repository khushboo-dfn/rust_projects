use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match  args.next() {
            Some(args) => args,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(args) => args,
            None => return Err("Didnt get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents )
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ust";
        let contents = "\
Rust:testsffwl
fowtfiwe";
        assert_eq!(
            vec!["Rust:testsffwl"],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "fowtfiwe";
        let contents = "\
Rust:testsffwl
fowtfiwe";
        assert_eq!(vec!["fowtfiwe"],
            search(query, contents)
        );
    }

    #[test]
    fn case_in_sensitive() {
        let query = "RuSt";
        let contents = "\
Rust:testsffwl
fowtfiwe";
        assert_eq!(vec!["Rust:testsffwl"],
            search_case_insensitive(query, contents)
        );
    }
}