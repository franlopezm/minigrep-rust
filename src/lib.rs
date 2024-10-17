use std::{error::Error, fs};

pub struct Arguments {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub count_line: bool,
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }

        let mut args_clone = args.clone();

        args_clone.remove(0); // Remove first line with program path
        let file_path = args_clone.pop().unwrap();
        let query = args_clone.pop().unwrap_or_default();

        let mut ignore_case = false;
        let mut count_line = false;

        for arg in args_clone.iter() {
            if arg.contains("-") {
                if arg.contains("i") {
                    ignore_case = true;
                }

                if arg.contains("c") {
                    count_line = true;
                }
            }
        }

        Ok(Self {
            query,
            file_path,
            ignore_case,
            count_line,
        })
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    if args.count_line {
        println!("{}", results.len())
    } else {
        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
