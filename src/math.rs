use errors::Error;
use reader::lists::ConsIter;
use types::{ValuePtr, Fixnum};

pub fn add(mut args: ConsIter) -> Result<ValuePtr, Error> {
    let a = args.next();
    let b = args.next();

    a.and_then(|a| a.map(|b| {
        match a {
            Value::Fixnum(a) => {
                match b {
                    Value::Fixnum(b) => {
                        Ok(ValuePtr::from(Value::from(Fixnum::new(a.value + b.value))))
                    },
                    _ => Err(Error::MismatchedTypes)
                }
            },
            _ => Err(Error::MismatchedTypes)
        }
    }))
}
