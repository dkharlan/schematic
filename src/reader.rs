use std::convert::TryFrom;

use pest::Parser;

use errors;
use types::{Value, Atom, Symbol, Fixnum, Str, Boolean};

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

pub fn read(input: &str) -> Result<Value, errors::Error> {
    let mut pairs = ExampleParser::parse(Rule::expression, &input)
        .unwrap_or_else(|e| panic!("{}", e));

    let first_pair = pairs.next();
    let second_pair = pairs.next();

    // TODO handle multiple values
    if let Some(_) = second_pair {
        println!(" WARN: Multiple pairs from parser");
    }

    match first_pair {
        Some(pair) => {
            let span = pair.clone().into_span();
            let token_string = span.as_str().to_string();
            match pair.as_rule() {
                Rule::symbol => {
                    let symbol: Atom = Symbol::from(token_string).into();
                    Ok(symbol.into())
                },
                Rule::integer => {
                    let fixnum_opt = Fixnum::try_from(token_string).map(|i| i.into());
                    fixnum_opt.map(|a: Atom| a.into())
                },
                Rule::boolean => {
                    let boolean_opt = Boolean::try_from(token_string).map(|b| b.into());
                    boolean_opt.map(|a: Atom| a.into())
                },
                Rule::string => {
                    let str: Atom = Str::from(token_string).into();
                    Ok(str.into())
                },
                _ => Err(errors::Error::UnknownToken)
            }
        },
        None => {
            Err(errors::Error::EmptyValues)
        }
    }

    //Rule::list => {
        //    println!("list = {}", token_str);
        //    for thing in pair.into_inner() {
        //        println!("thing = {:?}", thing);
        //    }
        //},
}
