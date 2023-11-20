use super::Location;

pub struct Scanner {
    cursor: usize,
    loc: Location,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(s: &str) -> Scanner {
        Scanner {
            cursor: 0,
            loc: Location { line: 0, col: 0 },
            characters: s.chars().collect(),
        }
    }

    pub fn index(&self) -> usize {
        self.cursor
    }

    pub fn loc(&self) -> Location {
        self.loc
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn not_empty(&self) -> bool {
        self.len() > 0
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

            if c == '\n' {
                self.loc.line += 1;
                self.loc.col = 0;
            } else {
                self.loc.col += 1;
            }

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
                self.next();
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
    !"'.,`()\\\"".contains(c) && !c.is_whitespace()
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LispLexingError {
    #[error("trailing garbage")]
    TrailingGarbage,
    #[error("empty program")]
    Empty,
    #[error("unknown character '{0}'")]
    UnKnownChar(char),
    #[error("unmatched '{0}'")]
    UnMatched(char),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    Number(f64),
    Symbol(String),
    String(String),
    Quote,
    Dot,
    Quasiquote,
    Unquote,
}

#[derive(Clone)]
pub struct Token {
    pub loc: Location,
    pub inner: TokenType,
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, (LispLexingError, Location)> {
    let mut scanner = Scanner::new(expression);

    let mut tokens: Vec<Token> = Vec::new();

    // list of indexes of unmatched parentheses
    let mut unmatched_parens: Vec<Location> = Vec::new();

    while scanner.not_empty() {
        if scanner.take(';').is_some() {
            scanner.take_until(|x| x == '\n');
        } else if scanner.next_is('(') {
            let token = Token {
                loc: scanner.loc(),
                inner: TokenType::OpenParen,
            };

            let _ = scanner.next();

            unmatched_parens.push(token.loc);
            tokens.push(token);
        } else if scanner.next_is(')') {
            if !unmatched_parens.is_empty() {
                unmatched_parens.pop();
                let token = Token {
                    loc: scanner.loc(),
                    inner: TokenType::CloseParen,
                };

                tokens.push(token);
                let _ = scanner.next();
            } else {
                return Err((LispLexingError::UnMatched(')'), scanner.loc()));
            }
        } else if scanner.next_matches(char::is_numeric) {
            let token_start = scanner.loc();
            let mut num = scanner.take_while(char::is_numeric).unwrap();
            if scanner.take('.').is_some() {
                num.push('.');
                num.push_str(&scanner.take_while(char::is_numeric).unwrap());
                if !(scanner.next_matches(char::is_whitespace) || scanner.next_is_one_of("()")) {
                    return Err((LispLexingError::TrailingGarbage, scanner.loc()));
                }
                let token = Token {
                    loc: token_start,
                    inner: TokenType::Number(num.parse().unwrap()),
                };

                tokens.push(token);
            } else if scanner.is_empty()
                || scanner.next_matches(char::is_whitespace)
                || scanner.next_is_one_of("()")
            {
                let token = Token {
                    loc: token_start,
                    inner: TokenType::Number(num.parse().unwrap()),
                };

                tokens.push(token);
            } else {
                return Err((LispLexingError::TrailingGarbage, scanner.loc()));
            }
        } else if scanner.next_matches(is_symbolic) {
            let token_start = scanner.loc();
            let name = scanner.take_while(is_symbolic).unwrap();
            if scanner.next_matches(char::is_whitespace)
                || scanner.next_is_one_of("()")
                || scanner.is_empty()
            {
                let token = Token {
                    loc: token_start,
                    inner: TokenType::Symbol(name),
                };

                tokens.push(token);
            } else {
                return Err((LispLexingError::TrailingGarbage, scanner.loc()));
            }
        } else if scanner.next_is('"') {
            let first_double_quote = scanner.loc(); // we consumed the character so need to backtrack
            let _ = scanner.next();
            let text = scanner.take_until(|x| x == '"' || x == '\n').unwrap(); // don't allow multiline strings like this.
            if scanner.next_is('"') {
                scanner.next();
            } else {
                return Err((LispLexingError::UnMatched('"'), first_double_quote));
            }

            let token = Token {
                loc: first_double_quote,
                inner: TokenType::String(text),
            };

            tokens.push(token);
        } else if scanner.next_is('\'') {
            let token_loc = scanner.loc();
            let _ = scanner.next();
            let token = Token {
                loc: token_loc,
                inner: TokenType::Quote,
            };

            tokens.push(token);
	} else if scanner.next_is('.') {
	    let token_loc = scanner.loc();
	    let _ = scanner.next();
	    let token = Token {
		loc: token_loc,
		inner: TokenType::Dot,
	    };

	    tokens.push(token)
        } else if scanner.next_is('`') {
            let token_loc = scanner.loc();
            let _ = scanner.next();
            let token = Token {
                loc: token_loc,
                inner: TokenType::Quasiquote,
            };

            tokens.push(token);
        } else if scanner.next_is(',') {
            let token_loc = scanner.loc();
            let _ = scanner.next();
            let token = Token {
                loc: token_loc,
                inner: TokenType::Unquote,
            };

            tokens.push(token);
	} else if scanner.next_is('.') {
	    let token_loc = scanner.loc();
	    let _ = scanner.next();
	    let token = Token {
		loc: token_loc,
		inner: TokenType::Unquote,
	    };

	    tokens.push(token);
        } else if scanner.next_matches(char::is_whitespace) {
            let _ = scanner.take_while(char::is_whitespace);
        } else {
            return Err((
                LispLexingError::UnKnownChar(scanner.peek().unwrap()),
                scanner.loc(),
            ));
        }
    }

    if !unmatched_parens.is_empty() {
        return Err((
            LispLexingError::UnMatched('('),
            unmatched_parens.pop().unwrap(),
        ));
    }

    if tokens.is_empty() {
        Err((LispLexingError::Empty, scanner.loc()))
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
                if loc.line >= 3 {
                    loc.line - 3
                } else {
                    0
                }
            };

            for number in min_line..=loc.line {
                eprintln!(
                    "{} | {}",
                    number + 1,
                    expression.lines().nth(number).unwrap()
                );
            }

            // go past the padding with line numbers and stuff
            let padding_len = format!("{} | ", loc.line).len();
            for _ in 0..padding_len {
                eprint!(" ");
            }

            for _ in 0..loc.col {
                eprint!(" ");
            }

            eprintln!("^");

            eprintln!(
                "error: {} on line {} column {}",
                e,
                loc.line + 1,
                loc.col + 1
            );

            None
        }
    }
}
