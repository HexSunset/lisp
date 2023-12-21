use std::io::Read;

mod token;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("USAGE: {} FILENAME", args.nth(0).unwrap());
        std::process::exit(1);
    }

    let fname = std::env::args().nth(1).unwrap();

    let mut file = std::fs::File::open(&fname).unwrap();

    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    let tokens = match token::tokenize_or_print_error(&program) {
        Some(t) => t,
        None => std::process::exit(1),
    };

    println!("{}:", fname);
    for token in tokens {
        println!("{} {:?}", token.loc, token.inner);
    }
}
