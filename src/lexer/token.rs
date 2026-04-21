use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus,  // +
    Minus, // -
    Slash, // /
    Star,  // *

    LBracket,
    RBracket,

    Number(f64),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenType, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    pub fn plus(line: usize, column: usize) -> Self {
        Token::new(TokenType::Plus, line, column)
    }

    pub fn minus(line: usize, column: usize) -> Self {
        Token::new(TokenType::Minus, line, column)
    }

    pub fn star(line: usize, column: usize) -> Self {
        Token::new(TokenType::Star, line, column)
    }
    pub fn slash(line: usize, column: usize) -> Self {
        Token::new(TokenType::Slash, line, column)
    }
    pub fn number(num: f64, line: usize, column: usize) -> Self {
        Token::new(TokenType::Number(num), line, column)
    }
    pub fn eof() -> Self {
        Token::new(TokenType::Eof, 0, 0)
    }
    pub fn lbracket(line: usize, column: usize) -> Self {
        Token::new(TokenType::LBracket, line, column)
    }
    pub fn rbracket(line: usize, column: usize) -> Self {
        Token::new(TokenType::RBracket, line, column)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = format!("{}:{}", self.line, self.column);
        match self.kind {
            TokenType::Minus => write!(f, "{} {}", "-", pos),
            TokenType::Plus => write!(f, "{} {}", "+", pos),
            TokenType::Star => write!(f, "{} {}", "*", pos),
            TokenType::Slash => write!(f, "{} {}", "/", pos),
            TokenType::LBracket => write!(f, "{} {}", "(", pos),
            TokenType::RBracket => write!(f, "{} {}", ")", pos),
            TokenType::Number(num) => write!(f, "{} {}", num, pos),
            TokenType::Eof => write!(f, "{}", "EOF"),
        }
    }
}
