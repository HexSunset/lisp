#[derive(Clone)]
pub enum Value {
    Symbol(String),
    Text(String),
    Number(f64),
    Cons(Cons),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Value::*;
        match self.clone() {
            Symbol(s) => write!(f, "{}", s),
            Text(s) => write!(f, "{:?}", s),
            Number(n) => write!(f, "{}", n),
            Nil => write!(f, "nil"),
            Cons(c) => {
                if self.is_list() {
                    let elems: Vec<Value> = Vec::try_from(self.clone()).unwrap();
                    let s: Vec<String> = elems.iter().map(|x| format!("{}", x)).collect();
                    let s: String = s.join(" ");

                    write!(f, "({})", s)
                } else {
                    write!(f, "({} . {})", c.car, c.cdr)
                }
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
            Value::Cons(cons) => match *cons.cdr {
                Value::Cons(_) => cons.cdr.is_list(),
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
