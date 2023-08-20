mod lisp;

use lisp::*;

fn main() {
    let numbers = [Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    let l = Value::from(&numbers[..]);

    println!("{}", l);
}
