use std::{fs, error::Error};

pub struct Arguments {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool
}

impl Arguments {
  pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments.");
      }

      let mut args_clone = args.clone();
      args_clone.remove(0);
      let file_path = args_clone.pop().unwrap();

      let mut query: String = String::from(""); // args[1].clone();
      let mut ignore_case = false; // = args[2].clone();

      for arg in args_clone.iter() {
        if arg.replace("-", "") == "i" {
          ignore_case = true;
        } else {
          query = arg.clone();
        }
      }

      Ok(Self { query, file_path, ignore_case })
  }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(args.file_path)?;

  let results = if args.ignore_case {
    search_case_insensitive(&args.query, &contents)
  } else {
    search(&args.query, &contents)
  };

  for line in results {
    println!("{line}");
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