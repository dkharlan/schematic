extern crate pest;

#[macro_use]
extern crate pest_derive;

use std::io;
use std::io::{BufRead, Write};

use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

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

        let pairs = ExampleParser::parse(Rule::expression, &line)
            .unwrap_or_else(|e| panic!("{}", e));

        // FIXME? should only be one pair?
        for pair in pairs {
            let span = pair.clone().into_span();
            let token_str = span.as_str();
            match pair.as_rule() {
                Rule::symbol => println!("symbol = {:?}", token_str),
                Rule::integer => match token_str.parse::<i64>() {
                    Ok(i) => println!("integer = {}", i),
                    Err(_) => println!("larger than i64 not yet supported (input was {:?})", token_str)
                },
                Rule::boolean => match token_str.as_ref() {
                    "true" => println!("boolean = {}", true),
                    "false" => println!("boolean = {}", false),
                    _ => unreachable!()
                },
                Rule::string => println!("string = {}", token_str),
                Rule::list => {
                    println!("list = {}", token_str);
                    for thing in pair.into_inner() {
                        println!("thing = {:?}", thing);
                    }
                },
                _ => println!("UNKNOWN pattern = {}", token_str)   // TODO? change to unreachable!() ?
            }
        }
    }
}
