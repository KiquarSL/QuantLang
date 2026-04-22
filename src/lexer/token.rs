use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus,      // +
    Minus,     // -
    Slash,     // /
    Star,      // *
    LParent,   // (
    RParent,   // )
    Assign,    // =
    Semicolon, // ;
    Keyword(String),
    Type(String),
    Id(String),
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

    pub fn lparent(line: usize, column: usize) -> Self {
        Token::new(TokenType::LParent, line, column)
    }

    pub fn rparent(line: usize, column: usize) -> Self {
        Token::new(TokenType::RParent, line, column)
    }

    pub fn assign(line: usize, column: usize) -> Self {
        Token::new(TokenType::Assign, line, column)
    }

    // Токены с данными
    pub fn keyword(name: String, line: usize, column: usize) -> Self {
        Token::new(TokenType::Keyword(name), line, column)
    }

    pub fn data_type(name: String, line: usize, column: usize) -> Self {
        Token::new(TokenType::Type(name), line, column)
    }

    pub fn ident(name: String, line: usize, column: usize) -> Self {
        Token::new(TokenType::Id(name), line, column)
    }

    pub fn number(num: f64, line: usize, column: usize) -> Self {
        Token::new(TokenType::Number(num), line, column)
    }

    pub fn semicolon(line: usize, column: usize) -> Self {
        Token::new(TokenType::Semicolon, line, column)
    }

    pub fn eof() -> Self {
        Token::new(TokenType::Eof, 0, 0)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pos = format!("{}:{}", self.line, self.column);
        match &self.kind {
            TokenType::Minus => write!(f, "- {}", pos),
            TokenType::Plus => write!(f, "+ {}", pos),
            TokenType::Star => write!(f, "* {}", pos),
            TokenType::Slash => write!(f, "/ {}", pos),
            TokenType::LParent => write!(f, "( {}", pos),
            TokenType::RParent => write!(f, ") {}", pos),
            TokenType::Assign => write!(f, "= {}", pos),
            TokenType::Semicolon => write!(f, "; {}", pos),
            TokenType::Type(name) => write!(f, "type:{} {}", name, pos),
            TokenType::Id(name) => write!(f, "id:{} {}", name, pos),
            TokenType::Keyword(name) => write!(f, "kw:{} {}", name, pos),
            TokenType::Number(num) => write!(f, "{} {}", num, pos),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}
