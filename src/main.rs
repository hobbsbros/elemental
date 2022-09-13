//! Main executable for the Elemental language interpreter.

use std::{
    io::{
        self,
        Write,
    },
    collections::HashMap,
};

use colored::*;

use elemental::interpret;
use elemental::error::*;

const VERSION: &str = "0.1.0";

fn main() -> ! {
    // Welcome message
    println!("{}\nVersion {}", "The Elemental Interpreter".truecolor(255, 140, 0).bold(), VERSION);

    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Store a list of variables in the program
    let mut variables = HashMap::new();

    loop {
        // Prompt the user
        print!(">>> ");
        match stdout.flush() {
            Ok(_) => (),
            Err(_) => throw(CouldNotFlushOutput),
        };

        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => throw(CouldNotReadStdin),
        };

        let (expression, is_silent) = interpret(&mut variables, input.to_owned());

        // Only if it is not "silent", display output
        if !is_silent {
            let output = format!(
                "{}",
                expression,
            );
            println!("\n{}\n", output);
        }

        input.clear();
    }
}