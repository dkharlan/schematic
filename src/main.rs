#![feature(try_from)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod reader;
mod errors;
mod types;
mod util;

use std::io;
use std::io::{BufRead, Write};

use util::FromRef;
use reader::read;

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

        println!(" DEBUG: raw input = {}", line);

        match read(&line) {
            Ok(value_ptr) => {

                let value_ref = &value_ptr.obj;

                // TODO remove me
                println!(" DEBUG\n{:#?}\n DEBUG\n", value_ptr);
                // TODO remove me

                let repr = String::from_ref(value_ref);
                println!(" ==> {}", repr);
            },
            Err(e) => {
                println!(" ERROR: {:?}", e);
            }
        };
    }

    // TODO print pithy quote
    println!("\nGoodbye.");
}
