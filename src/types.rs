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

impl Into<Atom> for Symbol {
    fn into(self) -> Atom {
        Atom::Symbol(self)
    }
}

impl Into<Atom> for Str {
    fn into(self) -> Atom {
        Atom::String(self)
    }
}

impl Into<Atom> for Fixnum {
    fn into(self) -> Atom {
        Atom::Fixnum(self)
    }
}

impl Into<Atom> for Boolean {
    fn into(self) -> Atom {
        Atom::Boolean(self)
    }
}

#[derive(Debug)]
pub struct Cell {
    left: Box<Value>,
    right: Box<Value>
}

#[derive(Debug)]
pub enum Value {
    Nil,
    Atom(Atom),
    Cell(Cell)
}

impl Into<Value> for Atom {
    fn into(self) -> Value {
        Value::Atom(self)
    }
}

impl Into<Value> for Cell {
    fn into(self) -> Value {
        Value::Cell(self)
    }
}
