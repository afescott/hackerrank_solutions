#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim().split_once(' ') {
            Some((command, args)) => match command {
                "type" => match args {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", args),
                    _ => println!("{}: not found", args),
                },
                input if input.starts_with("echo ") => println!("{}", &input[5..]),
                "exit 0" => break,
                "echo" => println!("{}", input),
                &_ => println!("{}: command not found", input.trim()),
            },
            None => println!("{}: command not found", input.trim()),
        }
    }
}
