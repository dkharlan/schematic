use std::sync::Arc;

use errors::Error;
use util::FromRef;

// FIXME this is pretty much all public for now
// this will change once I settle on interfaces (both from the Rust and Lisp sides), at
// which point I'll probably refactor quite a bit

#[derive(Debug)]
pub struct Symbol {
    pub value: String
}

impl Symbol {
    pub fn new(value: String) -> Self {
        Symbol {
            value
        }
    }
}

#[derive(Debug)]
pub struct Str {
    pub value: String
}

#[derive(Debug)]
pub struct Fixnum {
    pub value: i64
}

impl Fixnum {
    pub fn new(value: i64) -> Self {
        Fixnum {
            value
        }
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub value: bool
}

type Callable = fn(ConsIter) -> Result<ValuePtr, Error>;

#[derive(Debug)]
pub enum Atom {
    Symbol(Symbol),
    String(Str),
    Fixnum(Fixnum),
    Boolean(Boolean),
    Func(Callable)
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

impl From<Callable> for Atom {
    fn from(c: Callable) -> Self {
        Atom::Callable(c)
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

// TODO should left and right be boxed?
#[derive(Debug)]
pub struct Cons {
    pub left: Value,
    pub right: Value
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Atom(Arc<Atom>),
    Cons(Arc<Cons>)
}

impl From<Atom> for Value {
    fn from(atom: Atom) -> Self {
        Value::Atom(Arc::new(atom))
    }
}

impl From<Cons> for Value {
    fn from(cons: Cons) -> Self {
        Value::Cons(Arc::new(cons))
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
