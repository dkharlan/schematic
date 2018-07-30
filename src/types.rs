use std::rc::Rc;

use util::FromRef;
use errors;

// FIXME this is pretty much all public for now
// this will change once I settle on interfaces (both from the Rust and Lisp sides), at
// which point I'll probably refactor quite a bit

#[derive(Debug)]
pub struct Symbol {
    pub value: String
}

#[derive(Debug)]
pub struct Str {
    pub value: String
}

#[derive(Debug)]
pub struct Fixnum {
    pub value: i64
}

#[derive(Debug)]
pub struct Boolean {
    pub value: bool
}

#[derive(Debug)]
pub enum Atom {
    Symbol(Symbol),
    String(Str),
    Fixnum(Fixnum),
    Boolean(Boolean)
}

impl From<Symbol> for Atom {
    fn from(symbol: Symbol) -> Self {
        Atom::Symbol(symbol)
    }
}

impl From<Str> for Atom {
    fn from(s: Str) -> Self {
        Atom::String(s)
    }
}

impl From<Fixnum> for Atom {
    fn from(num: Fixnum) -> Self {
        Atom::Fixnum(num)
    }
}

impl From<Boolean> for Atom {
    fn from(b: Boolean) -> Self {
        Atom::Boolean(b)
    }
}

impl FromRef<Atom> for String {
    fn from_ref(atom: &Atom) -> Self {
        match atom {
            &Atom::Symbol(ref s) => s.value.clone(),
            &Atom::String(ref s) => s.value.clone(),
            &Atom::Fixnum(ref f) => f.value.to_string(),
            &Atom::Boolean(ref b) => b.value.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Cons {
    pub left: Value,
    pub right: Value
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Atom(Rc<Atom>),
    Cons(Rc<Cons>)
}

impl From<Atom> for Value {
    fn from(atom: Atom) -> Self {
        Value::Atom(Rc::new(atom))
    }
}

impl From<Cons> for Value {
    fn from(cons: Cons) -> Self {
        Value::Cons(Rc::new(cons))
    }
}

impl FromRef<Value> for String {
    fn from_ref(value_ref: &Value) -> Self {
        match value_ref {
            &Value::Nil => "nil".to_owned(),
            &Value::Atom(ref atom_rc) => String::from_ref(&**atom_rc),
            &Value::Cons(_) => {
                let mut reprs = Vec::new();
                for element in value_ref.iter() {
                    let repr = String::from_ref(element);
                    reprs.push(repr);
                }
                format!("({})", reprs.join(" "))
            }
        }
    }
}

// so we can point to the heap from the stack
#[derive(Debug, Clone)]
pub struct ValuePtr {
    pub obj: Value
}

impl From<Value> for ValuePtr {
    fn from(value: Value) -> Self {
        ValuePtr {
            obj: value
        }
    }
}

impl ValuePtr {
    pub fn new() -> Self {
        ValuePtr {
            obj: Value::Nil
        }
    }
}

pub fn cons(list: &ValuePtr, value: Value) -> ValuePtr {
    ValuePtr {
        obj: Value::Cons(Rc::new(Cons {
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
