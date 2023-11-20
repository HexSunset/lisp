pub mod parse;
pub mod token;

#[derive(Clone, Debug)]
pub enum Value {
    Symbol(String),
    String(String),
    Number(f64),
    Cons(Cons),
    Function(Cons),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	match self.clone() {
	    Value::Symbol(s) => write!(f, "{}", s),
	    Value::String(s) => write!(f, "{:?}", s),
	    Value::Number(n) => write!(f, "{}", n),
	    Value::Nil => write!(f, "nil"),
	    Value::Cons(c) | Value::Function(c) => {
		let mut cons: Cons = c.clone();
		write!(f, "({}", *cons.car)?;

		loop {
		    if let Value::Cons(inner) | Value::Function(inner) = *cons.cdr {
			write!(f, " {}", inner.car)?;
			cons = inner;
		    } else if let Value::Nil = *cons.cdr {
			write!(f, ")")?;
			break;
		    } else {
			write!(f, " . {})", *cons.cdr)?;
			break;
		    }
		}
		Ok(())
	    }
	}
    }
}

#[derive(Clone, Debug)]
pub struct Cons {
    car: Box<Value>,
    cdr: Box<Value>, // TODO: This should technically be a Option<Box<Value>> so we don't allocate a nil value.
}

impl Value {
    pub fn is_list(&self) -> bool {
	match self {
	    Value::Cons(cons) => match *cons.cdr {
		Value::Cons(_) | Value::Nil => true,
		_ => false,
	    },
	    _ => false,
	}
    }

    pub fn is_pair(&self) -> bool {
	match self {
	    Value::Cons(c) => {
		if let Value::Cons(_) | Value::Nil = *c.cdr {
		    false
		} else {
		    true
		}
	    },
	    _ => false,
	}
    }

    pub fn cons(car: Value, cdr: Value) -> Value {
	let car = Box::new(car);
	let cdr = Box::new(cdr);

	Value::Cons(Cons { car, cdr })
    }

    pub fn cons_pure(car: Value, cdr: Value) -> Cons {
	let car = Box::new(car);
	let cdr = Box::new(cdr);
	Cons { car, cdr }
    }

    pub fn vec_to_list(elements: Vec<Value>) -> Value {
	let mut out_val = Value::Nil;
	let mut elements = elements.clone();
	elements.reverse();

	for element in elements {
	    out_val = Value::cons(element, out_val);
	}

	out_val
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    line: usize,
    col: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	write!(f, "{}:{}", self.line, self.col)
    }
}
