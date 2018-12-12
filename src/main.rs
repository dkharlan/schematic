#![feature(try_from)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

mod reader;
mod errors;
mod types;
mod util;
mod math;

use std::io;
use std::io::{BufRead, Write};
use std::env;
use std::collections::HashMap;

use util::FromRef;
use types::{ValuePtr, Value, ConsIter, Atom, Symbol};
use errors::Error;

type Environment = HashMap<Symbol, ValuePtr>;

fn create_default_environment() -> Environment {
    let mut environment = HashMap::new();

    environment.insert(Symbol::new("+".to_string()), math::add);

    // TODO just for testing
    environment.insert(Symbol::new("test".to_string()), 100);

    environment
}

fn eval(mut env: Environment, mut items: ConsIter) -> Result<(Environment, ValuePtr), Error> {
    match items.next() {
        Some(value) => {
            match value {
                Value::Nil => {
                    (env, value)
                },
                Value::Atom(ref atom) => {
                    match atom {
                        Atom::Symbol(symbol) => {
                            (env, env.get(symbol))
                        },
                        _ => (env, atom)
                    }
                },
                Value::Cons(cons) => {
                    unimplemented!()
                }
            }
        },
        None => (env, ValuePtr::new())
    }
}

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

        let mut environment = HashMap::new();

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
