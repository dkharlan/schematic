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
use std::env;

use util::FromRef;
use reader::read;

fn main() {

    let debug_mode = match env::var("DEBUG") {
        Ok(val) => match val.to_lowercase().as_ref() {
            "true" => true,
            "false" => false,
            _ => false
        },
        Err(_) => false
    };

    if debug_mode {
        println!("Debug mode enabled.");
    }

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

        if debug_mode {
            println!(" DEBUG: raw input = {}", line);
        }

        match read(&line) {
            Ok(value_ptr) => {
                let value_ref = &value_ptr.obj;

                if debug_mode {
                    println!(" DEBUG\n  ;;; val begins here\n{:#?}\n  ;;; val ends here\n DEBUG\n", value_ptr);
                }

                println!(" ==> {}", String::from_ref(value_ref));
            },
            Err(e) => {
                println!(" ERROR: {:?}", e);
            }
        };
    }

    // TODO print pithy quote
    println!("\nGoodbye.");
}
