pub mod token;
pub mod parse;

#[derive(Debug, Copy, Clone)]
pub struct Location {
    line: usize,
    col: usize,
}

#[derive(Clone)]
struct Cons {
    // use options so we don't allocate Nil values
    car: Option<Box<Value>>,
    cdr: Option<Box<Value>>,
}

impl Cons {
    fn car(&self) -> Value {
	match &self.car {
	    None => Value::Nil,
	    Some(v) => *v.clone(),
	}
    }

    fn cdr(&self) -> Value {
	match &self.cdr {
	    None => Value::Nil,
	    Some(v) => *v.clone(),
	}
    }

    fn set_car(&mut self, v: Value) {
	match v {
	    Value::Nil => self.car = None,
	    _ => self.car = Some(Box::new(v)),
	}
    }

    fn set_cdr(&mut self, v: Value) {
	match v {
	    Value::Nil => self.cdr = None,
	    _ => self.cdr = Some(Box::new(v)),
	}
    }
}

fn cons(car: Value, cdr: Value) -> Value {
    Value::Cons(Cons {
	car: match car {
	    Value::Nil => None,
	    _ => Some(Box::new(car)),
	},
	cdr: match cdr {
	    Value::Nil => None,
	    _ => Some(Box::new(cdr)),
	},
    })
}

#[derive(Clone)]
enum Value {
    Cons(Cons),
    Symbol(String),
    Nil,
}
