#![allow(dead_code)]

#[derive(Clone)]
enum Value {
    Symbol(String),
    Text(String),
    Character(char),
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
            Character(c) => write!(f, "{:?}", c),
            Number(n) => write!(f, "{}", n),
            Nil => write!(f, "nil"),
            Cons(c) => {
                if is_list(self.clone()) {
                    let elems = list_to_vec(self.clone());
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

fn sym(s: &str) -> Value {
    if s == "nil" {
        Value::Nil
    } else {
        Value::Symbol(s.to_string())
    }
}

fn txt(t: &str) -> Value {
    Value::Text(t.to_string())
}

fn chr(c: char) -> Value {
    Value::Character(c)
}

fn num<F: Into<f64>>(n: F) -> Value {
    Value::Number(n.into())
}

fn nil() -> Value {
    Value::Nil
}

#[derive(Clone)]
struct Cons {
    car: Box<Value>,
    cdr: Box<Value>,
}

fn cons(car: Value, cdr: Value) -> Value {
    let car = Box::new(car);
    let cdr = Box::new(cdr);

    Value::Cons(Cons { car, cdr })
}

fn list(items: &[Value]) -> Value {
    let mut list = Value::Nil;

    let mut items = items.to_owned();
    items.reverse();

    for item in items {
        list = cons(item, list);
    }

    return list;
}

fn is_list(val: Value) -> bool {
    match val {
        Value::Cons(cons) => {
            if let Value::Cons(_) = *cons.cdr {
                true && is_list(*cons.cdr)
            } else if let Value::Nil = *cons.cdr {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn list_to_vec(val: Value) -> Vec<Value> {
    let mut list: Vec<Value> = Vec::new();

    match val.clone() {
        Value::Cons(cons) => match *(cons.cdr).clone() {
            Value::Cons(_) => {
                list.push(*cons.car);
                for item in list_to_vec(*cons.cdr) {
                    list.push(item);
                }
            }
            Value::Nil => list.push(*cons.car),
            _ => list.push(val),
        },
        _ => list.push(val),
    }

    list
}

fn main() {
    let l = list(&[num(1), num(2), num(3)]);

    println!("{}", l);
}
