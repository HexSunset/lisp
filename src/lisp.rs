pub mod parse;

#[derive(Clone)]
pub enum Value {
    Symbol(String),
    String(String),
    Number(f64),
    Pair(Cons),
    List(Cons),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.clone() {
            Value::Symbol(s) => write!(f, "{}", s),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
            Value::List(c) => {
                write!(f, "({}", c.car)?;

                let mut c = c.cdr;

                while let Value::List(cons) = *c.clone() {
                    write!(f, " {}", cons.car)?;
                    c = cons.cdr;
                }
                write!(f, ")")
            }
            Value::Pair(c) => {
                write!(f, "({} . {})", c.car, c.cdr)
            }
        }
    }
}

#[derive(Clone)]
pub struct Cons {
    car: Box<Value>,
    cdr: Box<Value>, // TODO: This should technically be a Option<Box<Value>> so we don't allocate a nil value.
}

impl From<&[Value]> for Value {
    fn from(items: &[Value]) -> Value {
        let mut list = Value::Nil;

        let mut items = items.to_owned();
        items.reverse();

        for item in items {
            list = Value::cons(item, list);
        }

        list
    }
}

impl Value {
    pub fn is_list(&self) -> bool {
        match self {
            Value::List(cons) => match *cons.cdr {
                Value::List(_) => cons.cdr.is_list(),
                Value::Nil => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn cons(car: Value, cdr: Value) -> Value {
        let car = Box::new(car);
        let cdr = Box::new(cdr);

        Value::Cons(Cons { car, cdr })
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = &'static str;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        if !val.is_list() {
            Err("Value is not a list")
        } else {
            let mut list: Vec<Value> = Vec::new();

            match val.clone() {
                Value::Cons(cons) => match *(cons.cdr).clone() {
                    Value::Cons(_) => {
                        list.push(*cons.car);
                        if let Ok(cdr_list) = Vec::try_from(*cons.cdr) {
                            for item in cdr_list {
                                list.push(item);
                            }
                        }
                    }
                    Value::Nil => list.push(*cons.car),
                    _ => list.push(val),
                },
                _ => list.push(val),
            }

            Ok(list)
        }
    }
}
