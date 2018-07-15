extern crate pest;

#[macro_use]
extern crate pest_derive;

use std::io;
use std::io::{BufRead, Write};

mod parser;
use parser::parse;

fn main() {

    let input = io::stdin();
    let mut lines = input.lock().lines();

    loop {
        print!("λ ");
        io::stdout().flush().expect("Error (I/O): Could not flush stdout");

        let mut line = String::new();
        match lines.next() {
            Some(result) => match result {
                Ok(l) => line = l,
                Err(e) => println!("Error (I/O): {}", e)
            },
            None => break // EOF
        }

        println!("DEBUG: raw input = {}", line);

        println!();

        parse(&line);
    }

    // TODO print pithy quote
    println!("\nGoodbye.");
}
