use util::FromRef;
use std::mem;

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

#[derive(Debug)]
pub enum Value {
    Nil,
    Atom(Box<Atom>),
    Cons(Box<Cons>)
}

impl From<Atom> for Value {
    fn from(atom: Atom) -> Self {
        Value::Atom(Box::new(atom))
    }
}

impl From<Cons> for Value {
    fn from(cons: Cons) -> Self {
        Value::Cons(Box::new(cons))
    }
}

impl FromRef<Value> for String {
    fn from_ref(value_ref: &Value) -> Self {
        match value_ref {
            &Value::Nil => "nil".to_owned(),
            &Value::Atom(ref boxed_atom) => String::from_ref(&**boxed_atom),
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
#[derive(Debug)]
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

// FIXME push and pop aren't part of the "protocol" of Value. they should be moved
impl ValuePtr {
    pub fn new() -> Self {
        ValuePtr {
            obj: Value::Nil
        }
    }

    // FIXME should this take a ValuePtr?
    pub fn push(&mut self, value: Value) {
        let cons = Box::new(Cons {
            left: value,
            right: mem::replace(&mut self.obj, Value::Nil)
        });
        self.obj = Value::Cons(cons);
    }

    pub fn pop(&mut self) -> Option<ValuePtr> {
        match mem::replace(&mut self.obj, Value::Nil) {
            Value::Nil => None,
            Value::Atom(_) => unimplemented!(),  // TODO what happens here?
            Value::Cons(boxed_cons) => {
                let cons = *boxed_cons;
                self.obj = cons.right;

                let mut ptr = ValuePtr::new();
                ptr.obj = cons.left;

                Some(ptr)
            }
        }
    }
}

impl<'a> Value {
    pub fn iter(&'a self) -> ConsIter {
        ConsIter {
            next: match self {
                &Value::Nil => None,
                &Value::Atom(_) => None, // FIXME should this be an error?
                &Value::Cons(ref boxed_cons) => Some(&*boxed_cons)
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
                &Value::Cons(ref boxed_cons) => Some(&boxed_cons)
            };

            &cons_ref.left
        })
    }
}
