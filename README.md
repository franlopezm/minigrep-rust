# Minigrep-rust

Grep is a command-line utility in Unix and Unix-like operating systems used for searching and manipulating text.

This implementation is part of the exercise in chapter 12 of the 'The Rust Programming Language' book.
> Exercise from https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html


### Usage
The program accepts two or more arguments:

- The filtering options, which can be **(Optional arguments)**:
  - `-i`, case insensitive matching. By default, **minigrep** is case sensitive.
  - `-c`, indicates the number of matching lines.
- The string to search for.
- The file to search in.

For example:

```sh
cargo run -- -c how mytext.txt
```

Will search the number of lines for the string how in mytext.txt and write the lines in the Standard output.