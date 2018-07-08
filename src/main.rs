extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

fn main() {

    let input  = "one 123";

    let pairs = ExampleParser::parse(Rule::thing, input)
        .unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        let span = pair.clone().into_span();

        // for token in pair.tokens() {
        //     println!("Token:   {:?}", token);
        // }

        println!("Rule:   {:?}", pair.as_rule());
        println!("Span:   {:?}", span);
        println!("Text:   {}", span.as_str());

        for inner_pair in pair.into_inner() {
            let inner_span = inner_pair.clone().into_span();
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:   {}", inner_span.as_str()),
                Rule::digit => println!("Digit:    {}", inner_span.as_str()),
                _ => unreachable!()
            }
        }
    }
}
