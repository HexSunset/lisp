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

    pub fn list_to_vec(&self) -> Vec<Value> {
        let mut elements = vec![];

	let mut head = self.clone();

	while let Value::List(c) = head.clone() {
	    elements.push(*c.car);
	    head = *c.cdr;
	}

        elements
    }

    pub fn cons(car: Value, cdr: Value) -> Value {
        let car = Box::new(car);
        let cdr = Box::new(cdr);

        if let Value::Nil = *cdr {
            Value::List(Cons { car, cdr })
        } else if cdr.is_list() {
            Value::List(Cons { car, cdr })
        } else {
            Value::Pair(Cons { car, cdr })
        }
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
