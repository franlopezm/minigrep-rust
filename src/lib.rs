use std::{fs, error::Error};

pub struct Arguments {
  pub query: String,
  pub file_path: String
}

impl Arguments {
  pub fn new(args: &[String]) -> Result<Self, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments.");
      }

      let query = args[1].clone();
      let file_path = args[2].clone();

      Ok(Self { query, file_path })
  }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(args.file_path)?;

  println!("Content {contents}");

  Ok(())
}
