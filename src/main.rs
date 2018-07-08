extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

fn main() {

    //let input = "1 a 4b b1 one true false 123 123234234234234234234234235436 -10 \"foo\"";     // errors?
    let input = "1 a 4 b1 one true false 123 123234234234234234234234235436 -10 \"foo\"";      // works
    //let input = "1 a 4 b1 one true false 123 123234234234234234234234235436 -10 \"foo\"";      // errors

    let pairs = ExampleParser::parse(Rule::things, input)
        .unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let span = pair.clone().into_span();
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
            Rule::string => println!("string = {}", token_str),
            _ => println!("UNKNOWN pattern = {}", token_str)
        }
    }
}
