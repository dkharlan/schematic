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

// TODO Look into refactoring this.  This is trickier than it appears, since it raises some semantic issues:
//    -] I can't use cool tricks like the reader's fold-and_then-map idiom unless I implement Iterator for
//       ValuePtr.
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

// TODO remove this and convert calls to car / cdr iteration
impl<'a> Value {
    pub fn iter(&'a self) -> ConsIter {
        ConsIter {
            next: match self {
                &Value::Nil => None,
                &Value::Atom(_) => None, // FIXME should this be an error?
                &Value::Cons(ref cons_rc) => Some(&*cons_rc)
            }
        }
    }
}

pub struct ConsIter<'a> {
    next: Option<&'a Cons>
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|cons_ref| {
            let right_cons_ref = &cons_ref.right;
            self.next = match right_cons_ref {
                &Value::Nil => None,
                &Value::Atom(_) => None,
                &Value::Cons(ref cons_rc) => Some(&cons_rc)
            };

            &cons_ref.left
        })
    }
}
