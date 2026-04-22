use crate::lexer::{Token, TokenType};
use crate::parser::{Expr, Op, Unary};
use crate::parser::{Stmt, is_assign_stmt, parse_assign_stmt};

type TT = TokenType;
type StmtT = Box<dyn Stmt>;

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    stmts: Vec<StmtT>,
}

impl Parser {
    fn new(tokens: Vec<Token>, stmts: Vec<StmtT>) -> Self {
        let tokens = tokens.clone();
        Self {
            pos: 0,
            tokens,
            stmts,
        }
    }

    pub fn get(&self, offset: i8) -> Token {
        let index = self.pos + offset as usize;
        if index < self.tokens.len() {
            self.tokens[index].clone()
        } else {
            Token::eof()
        }
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }

    pub fn match_type(&mut self, kind: TokenType) -> bool {
        let current = self.get(0);
        if kind == current.kind {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn parse_stmt(&mut self) {
        while !self.match_type(TT::Eof) {
            if is_assign_stmt(self) {
                let stmt: StmtT = parse_assign_stmt(self);
                self.stmts.push(stmt);
            } else {
                let token = self.get(0);
                panic!("Unknown statement in {}:{}", token.line, token.column);
            }
        }
    }

    pub fn expression(&mut self) -> Expr {
        return self.additive();
    }

    fn additive(&mut self) -> Expr {
        let expr = self.multiplicative();
        loop {
            if self.match_type(TT::Plus) {
                let right = self.multiplicative();
                return Expr::bin(expr, Op::Add, right);
            } else if self.match_type(TT::Minus) {
                let right = self.multiplicative();
                return Expr::bin(expr, Op::Sub, right);
            }
            break;
        }
        expr
    }

    fn multiplicative(&mut self) -> Expr {
        let expr = self.unary();
        loop {
            if self.match_type(TT::Star) {
                let right = self.unary();
                return Expr::bin(expr, Op::Mul, right);
            } else if self.match_type(TT::Slash) {
                let right = self.unary();
                return Expr::bin(expr, Op::Div, right);
            } else {
                break;
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        let token = self.get(0);
        match token.kind {
            TT::Minus => {
                self.next();
                Expr::unary(self.primary(), Unary::Negative)
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.get(0);
        match token.kind {
            TT::Number(n) => {
                self.next();
                let expr = Expr::number(n);
                expr
            }
            TT::LParent => {
                self.match_type(TT::LParent);
                let result = self.expression();
                self.match_type(TT::RParent);
                result
            }
            _ => panic!("Unknown expression in {}:{}", token.line, token.column),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<StmtT> {
    let stmts = Vec::new();
    let mut pr = Parser::new(tokens, stmts);
    pr.parse_stmt();
    pr.stmts
}
