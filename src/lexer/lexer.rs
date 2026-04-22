use crate::lexer::Token;

struct Lexer {
    pos: usize,
    tokens: Vec<Token>,
    text: Vec<char>,

    line: usize,
    column: usize,
}

impl Lexer {
    fn new(source: String) -> Self {
        let tokens = Vec::<Token>::new();
        let text = source.chars().collect();
        Self {
            pos: 0,
            line: 0,
            column: 0,
            tokens: tokens,
            text: text,
        }
    }

    fn is_valid(&self) -> bool {
        self.pos < self.text.len()
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn peek(&self, rel: i8) -> Option<char> {
        let index = self.pos + rel as usize;
        if index < self.text.len() {
            return Some(*self.text.get(index).unwrap());
        }
        return None;
    }

    fn next(&mut self) {
        self.pos += 1;
        if let Some(current) = self.peek(0) {
            if current == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }

    fn tokenize_number(&mut self, line: usize, column: usize) -> Token {
        let mut buffer = String::new();
        let mut has_dot = false;
        loop {
            if let Some(current) = self.peek(0) {
                if current.is_digit(10) || current == '_' {
                    buffer.push(current);
                } else if current == '.' {
                    if has_dot {
                        panic!("Number cannot have second dot! In {}:{}", line, column);
                    } else {
                        has_dot = true;
                        buffer.push(current);
                    }
                } else {
                    break;
                }
                self.next();
            } else {
                break;
            }
        }
        Token::number(buffer.parse().unwrap(), line, column)
    }

    fn tokenize_word(&mut self, line: usize, column: usize) -> Token {
        let mut buffer = String::new();
        loop {
            if let Some(current) = self.peek(0) {
                if current.is_alphabetic() || current.is_digit(10) || current == '_' {
                    buffer.push(current);
                } else {
                    break;
                }
                self.next();
            } else {
                break;
            }
        }
        if DATA_TYPES.contains(&buffer.as_str()) {
            Token::data_type(buffer.parse().unwrap(), line, column)
        } else if KEYWORDS.contains(&buffer.as_str()) {
            Token::keyword(buffer.parse().unwrap(), line, column)
        } else {
            Token::ident(buffer.parse().unwrap(), line, column)
        }
    }
}

const DATA_TYPES: &[&str] = &["i8", "i16", "i32", "i64", "f32", "f64", "bool", "char"];
const KEYWORDS: &[&str] = &["if", "else", "while", "fn"];

pub fn tokenize(source: String) -> Vec<Token> {
    let mut lx = Lexer::new(source);

    while lx.is_valid() {
        if let Some(current) = lx.peek(0) {
            match current {
                ' ' | '\n' => {
                    lx.next();
                    continue;
                }
                '=' => lx.push(Token::assign(lx.line, lx.column)),
                '+' => lx.push(Token::plus(lx.line, lx.column)),
                '-' => lx.push(Token::minus(lx.line, lx.column)),
                '*' => lx.push(Token::star(lx.line, lx.column)),
                ';' => lx.push(Token::semicolon(lx.line, lx.column)),
                '/' => lx.push(Token::slash(lx.line, lx.column)),
                '(' => lx.push(Token::lparent(lx.line, lx.column)),
                ')' => lx.push(Token::rparent(lx.line, lx.column)),
                _ if current.is_digit(10) => {
                    let token = lx.tokenize_number(lx.line, lx.column);
                    lx.push(token);
                    continue;
                }
                _ if current.is_alphabetic() => {
                    let token = lx.tokenize_word(lx.line, lx.column);
                    lx.push(token);
                    continue;
                }
                _ => panic!("Unknown char '{}' in {}:{}", current, lx.line, lx.column),
            }
            lx.next();
        } else {
            break;
        }
    }
    lx.push(Token::eof());
    lx.tokens
}
