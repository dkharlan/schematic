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
    fn from_ref(value: &Value) -> Self {
        match value {
            &Value::Nil => "nil".to_owned(),
            &Value::Atom(ref boxed_atom) => String::from_ref(&**boxed_atom),
            &Value::Cons(_) => unreachable!()
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

impl From<ValuePtr> for String {
    fn from(value_ptr: ValuePtr) -> Self {
        match value_ptr.obj {
            Value::Cons(_) => {
                let mut head = &value_ptr;
                let mut reprs = Vec::new();
                while let Some(element_ref) = car(&head) {
                    let repr = String::from_ref(element_ref);
                    reprs.push(repr);
                    (*head).obj = *element_ref;
                }
                format!("({})", reprs.join(" "))
            },
            _ => unimplemented!() //String::from(value_ptr.obj)
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

pub fn car(cons_ptr: &ValuePtr) -> Option<&Value> {
    match cons_ptr.obj {
        Value::Nil => None, // TODO None or Value::Nil?
        Value::Atom(_) => panic!(), // TODO should be an error, but not a panic
        Value::Cons(ref cons) => Some(&cons.left)
    }
}
