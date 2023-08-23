pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(s: &str) -> Scanner {
        Scanner {
            cursor: 0,
            characters: s.chars().collect(),
        }
    }

    pub fn index(&self) -> usize {
        self.cursor
    }

    pub fn loc(&self) -> Location {
        self.line_col(self.index())
    }

    pub fn line_col(&self, index: usize) -> Location {
        let mut line: usize = 1;
        let mut col: usize = 1;

        for (_, c) in (0..index).zip(&self.characters) {
            match *c {
                '\n' => {
                    line += 1;
                    col = 1;
                }
                _ => col += 1,
            }
        }

        Location(line, col)
    }

    pub fn is_empty(&self) -> bool {
        self.cursor == self.characters.len()
    }

    pub fn not_empty(&self) -> bool {
        self.cursor != self.characters.len()
    }

    pub fn len(&self) -> usize {
        self.characters.len() - self.cursor
    }

    pub fn next(&mut self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            let c = *self.characters.get(self.cursor).unwrap();
            self.cursor += 1;
            Some(c)
        }
    }

    pub fn take(&mut self, c: char) -> Option<char> {
        if self.is_empty() {
            None
        } else if self.next_is(c) {
            self.next()
        } else {
            None
        }
    }

    pub fn skip(&mut self, count: usize) -> Option<String> {
        if self.len() < count || count == 0 {
            None
        } else {
            let mut s = String::new();
            for _ in 0..count {
                s.push(self.next().unwrap());
            }
            Some(s)
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            let c = *self.characters.get(self.cursor).unwrap();
            //self.cursor += 1;
            Some(c)
        }
    }

    pub fn next_is(&self, c: char) -> bool {
        self.peek() == Some(c)
    }

    pub fn next_is_one_of(&self, chars: &str) -> bool {
        for c in chars.chars() {
            if self.next_is(c) {
                return true;
            }
        }

        false
    }

    pub fn next_matches<F>(&self, fun: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        match self.peek() {
            Some(c) => fun(c),
            None => false,
        }
    }

    pub fn take_while<F>(&mut self, fun: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        let mut out = String::new();
        while let Some(c) = self.peek() {
            if fun(c) {
                self.cursor += 1;
                out.push(c);
            } else {
                break;
            }
        }

        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    pub fn take_if<F>(&mut self, fun: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        match self.peek() {
            Some(c) => {
                if fun(c) {
                    self.next()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn take_until<F>(&mut self, fun: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        self.take_while(|x| !fun(x))
    }

    pub fn peek_while<F>(&mut self, fun: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        let mut out = String::new();
        while let Some(c) = self.peek() {
            if fun(c) {
                //self.cursor += 1;
                out.push(c);
            } else {
                break;
            }
        }

        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    pub fn peek_until<F>(&mut self, fun: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        self.peek_while(|x| !fun(x))
    }
}

pub fn is_symbolic(c: char) -> bool {
    !"',`()\\\"".contains(c) && !c.is_whitespace()
}

#[derive(Debug)]
pub struct Location(usize, usize);

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.0, self.1)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LispParseError {
    #[error("trailing garbage")]
    TrailingGarbage,
    #[error("empty program")]
    Empty,
    #[error("unknown character '{0}'")]
    UnKnownChar(char),
    #[error("unmatched '{0}'")]
    UnMatched(char),
}

#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Number(f64),
    Symbol(String),
    String(String),
    Quote,
    Quasiquote,
    Unquote,
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, (LispParseError, Location)> {
    let mut scanner = Scanner::new(expression);

    let mut tokens: Vec<Token> = Vec::new();

    // list of indexes of unmatched parentheses
    let mut unmatched_parens: Vec<usize> = Vec::new();

    while scanner.not_empty() {
        if scanner.take(';').is_some() {
            scanner.take_until(|x| x == '\n');
        } else if scanner.take('(').is_some() {
            tokens.push(Token::OpenParen);
            unmatched_parens.push(scanner.index() - 1);
        } else if scanner.next_is(')') {
            if !unmatched_parens.is_empty() {
                unmatched_parens.pop();
                tokens.push(Token::CloseParen);
                let _ = scanner.next();
            } else {
                return Err((LispParseError::UnMatched(')'), scanner.loc()));
            }
        } else if scanner.next_matches(char::is_numeric) {
            let mut num = scanner.take_while(char::is_numeric).unwrap();
            if scanner.take('.').is_some() {
                num.push('.');
                num.push_str(&scanner.take_while(char::is_numeric).unwrap());
                if !(scanner.next_matches(char::is_whitespace) || scanner.next_is_one_of("()")) {
                    return Err((LispParseError::TrailingGarbage, scanner.loc()));
                }
                let token = Token::Number(num.parse().unwrap());
                tokens.push(token);
            } else if scanner.is_empty()
                || scanner.next_matches(char::is_whitespace)
                || scanner.next_is_one_of("()")
            {
                let token = Token::Number(num.parse().unwrap());
                tokens.push(token);
            } else {
                return Err((LispParseError::TrailingGarbage, scanner.loc()));
            }
        } else if scanner.next_matches(is_symbolic) {
            let name = scanner.take_while(is_symbolic).unwrap();
            if scanner.next_matches(char::is_whitespace)
                || scanner.next_is_one_of("()")
                || scanner.is_empty()
            {
                tokens.push(Token::Symbol(name));
            } else {
                return Err((LispParseError::TrailingGarbage, scanner.loc()));
            }
        } else if scanner.take('"').is_some() {
            let first_double_quote = scanner.index() - 1; // we consumed the character so need to backtrack
            let text = scanner.take_until(|x| x == '"' || x == '\n').unwrap(); // don't allow multiline strings like this.
            if scanner.next_is('"') {
                scanner.skip(1);
            } else {
                return Err((
                    LispParseError::UnMatched('"'),
                    scanner.line_col(first_double_quote),
                ));
            }

            tokens.push(Token::String(text));
        } else if scanner.take('\'').is_some() {
            tokens.push(Token::Quote);
        } else if scanner.take('`').is_some() {
            tokens.push(Token::Quasiquote);
        } else if scanner.take(',').is_some() {
            tokens.push(Token::Unquote);
        } else if scanner.next_matches(char::is_whitespace) {
            let _ = scanner.take_while(char::is_whitespace);
        } else {
            return Err((
                LispParseError::UnKnownChar(scanner.peek().unwrap()),
                scanner.loc(),
            ));
        }
    }

    if !unmatched_parens.is_empty() {
        return Err((
            LispParseError::UnMatched('('),
            scanner.line_col(unmatched_parens.pop().unwrap()),
        ));
    }

    if tokens.is_empty() {
        Err((LispParseError::Empty, scanner.loc()))
    } else {
        Ok(tokens)
    }
}

// TODO: make a more verbose program type that includes filename and stuff
pub fn tokenize_or_print_error(expression: &str) -> Option<Vec<Token>> {
    match tokenize(expression) {
        Ok(tokens) => Some(tokens),
        Err((e, loc)) => {
            let min_line = {
                if loc.0 >= 3 {
                    loc.0 - 3
                } else {
                    0
                }
            };

            for number in min_line..loc.0 {
                eprintln!(
                    "{} | {}",
                    number + 1,
                    expression.lines().nth(number).unwrap()
                );
            }

            // go past the padding with line numbers and stuff
            let padding_len = format!("{} | ", loc.0).len();
            for _ in 0..padding_len {
                eprint!(" ");
            }

            for _ in 0..(loc.1 - 1) {
                eprint!(" ");
            }

            eprintln!("^");

            eprintln!("error: {} on line {} column {}", e, loc.0, loc.1);

            None
        }
    }
}
