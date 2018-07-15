use std::convert::TryFrom;

use pest::Parser;

use errors;
use types::{Atom, Symbol, Fixnum, Str, Boolean};

#[derive(Parser)]
#[grammar = "example.pest"]
struct ExampleParser;

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol {
            value: s
        }
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str {
            value: s
        }
    }
}

impl TryFrom<String> for Fixnum {
    type Error = errors::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.parse::<i64>() {
            Ok(i) => {
                Ok(Fixnum {
                    value: i
                })
            },
            Err(_) => Err(errors::Error::FixnumParsing)
        }
    }
}

impl TryFrom<String> for Boolean {
    type Error = errors::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "true" => Ok(Boolean { value: true }),
            "false" => Ok(Boolean { value: false }),
            _ => Err(errors::Error::BooleanParsing)
        }
    }
}

pub fn parse(input: &str) {
    let pairs = ExampleParser::parse(Rule::expression, &input)
        .unwrap_or_else(|e| panic!("{}", e));

    // FIXME? should only be one pair?
    for pair in pairs {
        let span = pair.clone().into_span();
        let token_string = span.as_str().to_string();

        let atom: Result<Atom, errors::Error> = match pair.as_rule() {
            Rule::symbol => Ok(Symbol::from(token_string).into()),
            Rule::integer => Fixnum::try_from(token_string).map(|i| i.into()),
            Rule::boolean => Boolean::try_from(token_string).map(|b| b.into()),
            Rule::string => Ok(Str::from(token_string).into()),
            _ => Err(errors::Error::UnknownToken)
        };

        match atom {
            Ok(atom) => {
                println!(" ==> {:?}", atom);
            },
            Err(e) => {
                println!(" ERROR: {:?}", e);
            }
        };

        //Rule::list => {
            //    println!("list = {}", token_str);
            //    for thing in pair.into_inner() {
            //        println!("thing = {:?}", thing);
            //    }
            //},
    }
}
