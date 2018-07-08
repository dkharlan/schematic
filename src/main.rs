extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

fn main() {

    //println!("check 1");

    let input  = "one true false 123 123234234234234234234234235436 -10 \"foo\"";
    //let input = "one";

    //println!("check 2");

    let pairs = ExampleParser::parse(Rule::main, input)
        .unwrap_or_else(|e| panic!("{}", e));

    //println!("check 3");

    for pair in pairs {
        let span = pair.clone().into_span();
        //println!("checking span {:?} => {:?}", pair.as_rule(), span.as_str());
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

    //println!("check 4");

    //let string_literal_pairs = ExampleParser::parse(Rule::string, "\"foobar\"").unwrap_or_else(|e| panic!("{}", e));
    //println!("{:?}", string_literal_pairs);

    //let stuff = ExampleParser::parse(Rule::main, "\"foobar\"").unwrap_or_else(|e| panic!("{}", e));
    //println!("{:?}", stuff);

    //println!("check 5");
}
