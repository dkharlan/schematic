extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

fn main() {

    let input  = "one true false 123";

    let pairs = ExampleParser::parse(Rule::thing, input)
        .unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let span = pair.clone().into_span();
        //println!("{:?} => {:?}", pair.as_rule(), span.as_str());
        let token_str = span.as_str();
        match pair.as_rule() {
            Rule::identifier => println!("ident = {:?}", token_str),
            Rule::integer => match token_str.parse::<i64>() {
                Ok(i) => println!("integer = {}", i),
                Err(_) => println!("larger than i64 not yet supported (input was {:?})", token_str)
            },
            Rule::boolean => match token_str.as_ref() {
                "true" => println!("boolean = {}", true),
                "false" => println!("boolean = {}", false),
                _ => unreachable!()
            },
            _ => println!("UNKNOWN pattern = {}", token_str)
        }
    }
}
