use std::sync::Arc;

use types::{Value, ValuePtr, Cons};
use errors;

pub fn cons(list: &ValuePtr, value: Value) -> ValuePtr {
    ValuePtr {
        obj: Value::Cons(Arc::new(Cons {
            left: value,
            right: list.obj.clone()
        }))
    }
}

pub fn car(list: &ValuePtr) -> Result<ValuePtr, errors::Error> {
    match list.obj {
        Value::Atom(_) => Err(errors::Error::MismatchedTypes),
        Value::Nil => Ok(ValuePtr {
            obj: Value::Nil
        }),
        Value::Cons(ref cons_rc) => Ok(ValuePtr {
            obj: cons_rc.left.clone()
        })
    }
}

pub fn cdr(list: &ValuePtr) -> Result<ValuePtr, errors::Error> {
    match list.obj {
        Value::Atom(_) => Err(errors::Error::MismatchedTypes),
        Value::Nil => Ok(ValuePtr {
            obj: Value::Nil
        }),
        Value::Cons(ref cons_rc) => Ok(ValuePtr {
            obj: cons_rc.right.clone()
        })
    }
}

pub fn reverse(list: &ValuePtr) -> Result<ValuePtr, errors::Error> {
    let mut head = list.clone();     // TODO suspect, look into why i'm cloning this
    let mut reversed_list = ValuePtr::new();

    loop {
        match car(&head) {
            Err(e) => { // TODO here
                return Err(e);
            }
            Ok(element) => {
                match cdr(&head) {
                    Err(e) => {
                        return Err(e); // TODO here
                    }
                    Ok(rest) => {
                        reversed_list = cons(&reversed_list, element.obj);
                        match rest.obj {
                            Value::Nil => break,
                            Value::Atom(_) => {
                                return Err(errors::Error::MismatchedTypes); // TODO here
                            }
                            Value::Cons(_) => {
                                head = rest;
                            }
                        }
                    }
                }
            }
        };
    }

    Ok(reversed_list)
}
