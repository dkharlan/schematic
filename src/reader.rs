use std::convert::TryFrom;

use pest::Parser;
use pest::iterators::Pair;

use errors;
use types::{Value, Atom, Cell, Symbol, Fixnum, Str, Boolean};

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

impl<'i> TryFrom<Pair<'i, Rule>> for Value {
    type Error = errors::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {

        let rule = pair.as_rule();
        let span = pair.clone().into_span();
        let token_string = span.as_str().to_string();

        match rule {
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
            Rule::list => {

                let mut head = Value::Nil;
                let prev = &mut head;

                let error;
                for mut pair in pair.into_inner() {

                    let current_value = match Value::try_from(pair) {
                        Ok(value) => value,
                        Err(e) => {
                            error = Some(e);
                            break
                        }
                    };

                    let current_cell = Cell {
                        left: current_value,
                        right: Value::Nil
                    };
                    let mut current = Value::Cell(Box::new(current_cell));

                    match *prev {
                        Value::Nil => {
                            *prev = current;
                        },
                        Value::Cell(cell) => {
                            (*cell).right = current;
                        },
                        Value::Atom(_) => {
                            error = Some(errors::Error::AttemptToConsAtom);
                            break
                        }
                    }

                    prev = &mut current;
                }

                if let Some(error) = error {
                    Err(error)
                }
                else {
                    Ok(head)
                }
            },
            _ => Err(errors::Error::UnknownToken)
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
        Some(pair) => Value::try_from(pair),
        None => Err(errors::Error::EmptyValues)
    }
}
