mod lisp;

use lisp::*;

fn main() {
    let l = slice_to_list(&[num(1), num(2), num(3)]);

    println!("{}", l);
}
