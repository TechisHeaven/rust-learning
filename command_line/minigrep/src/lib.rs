use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub value: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let value = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            value,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("with text:\n {:?}", contents);
    let result = if config.ignore_case {
        insensitive_search(&config.value, &contents)
    } else {
        search(&config.value, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

//insensitive search function
pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

//sensitive search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn content_query_test() {
        let query = "pro";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // insensitive query test
    #[test]
    fn insensitive_content_query_test() {
        let query = "Pro";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            insensitive_search(query, contents)
        );
    }
}
