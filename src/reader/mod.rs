mod lists;

use std::convert::TryFrom;

use pest::Parser;
use pest::iterators::Pair;

use errors;
use types::{Value, ValuePtr, Atom, Symbol, Fixnum, Str, Boolean};
use self::lists::{cons, reverse};

#[derive(Parser)]
#[grammar = "schematic.pest"]
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

impl<'i> TryFrom<Pair<'i, Rule>> for ValuePtr {
    type Error = errors::Error;

    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {

        let rule = pair.as_rule();
        let span = pair.clone().into_span();
        let token_string = span.as_str().to_string();

        // FIXME atomic types are constructed on the stack and then moved to the heap

        match rule {
            Rule::symbol => {
                let symbol: Atom = Symbol::from(token_string).into();
                let symbol_value: Value = symbol.into();
                Ok(symbol_value.into())
            },
            Rule::integer => {
                Fixnum::try_from(token_string)
                    .map(|i| i.into())
                    .map(|a: Atom| a.into())
                    .map(|v: Value| v.into())
            },
            Rule::boolean => {
                Boolean::try_from(token_string)
                    .map(|b| b.into())
                    .map(|a: Atom| a.into())
                    .map(|v: Value| v.into())
            },
            Rule::string => {
                let string: Atom = Str::from(token_string).into();
                let string_value: Value = string.into();
                Ok(string_value.into())
            },
            Rule::list => {
                // TODO - Make this more explicit that we're making a Result<ValuePtr::Cons> (so to speak) so that
                // reverse can be rewritten in terms of cons
                pair.into_inner()
                    .fold(Ok(ValuePtr::new()), |maybe_head, pair| {
                        ValuePtr::try_from(pair)
                            .and_then(|v| {
                                maybe_head.map(|head| cons(&head, v.obj))
                            })
                    })
                    .and_then(|v| reverse(&v))
            },
            _ => Err(errors::Error::UnknownToken)
        }
    }
}

pub fn read(input: &str) -> Result<ValuePtr, errors::Error> {
    let mut pairs = ExampleParser::parse(Rule::expression, &input)
        .unwrap_or_else(|e| panic!("{}", e));

    let first_pair = pairs.next();
    let second_pair = pairs.next();

    // TODO handle multiple values
    if let Some(_) = second_pair {
        println!(" WARN: Multiple pairs from parser");
    }

    match first_pair {
        Some(pair) => ValuePtr::try_from(pair),
        None => Err(errors::Error::EmptyValues)
    }
}
