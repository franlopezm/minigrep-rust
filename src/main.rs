use std::{env, process};
use minigrep::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(arguments) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
