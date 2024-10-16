use std::{env, process};
use minigrep::Arguments;

fn main() {
    let arguments = Arguments::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(arguments) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
