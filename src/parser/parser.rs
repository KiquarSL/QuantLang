use crate::lexer::{Token, TokenType};
use crate::parser::{BinExpr, Expr, NumExpr, Op, Unary, UnaryExpr};

type ExprT = Box<dyn Expr>;
type TT = TokenType;

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    exprs: Vec<ExprT>,
}

impl Parser {
    fn new(tokens: Vec<Token>, exprs: Vec<ExprT>) -> Self {
        let tokens = tokens.clone();
        Self {
            pos: 0,
            tokens,
            exprs,
        }
    }

    fn get(&self, offset: i8) -> Token {
        let index = self.pos + offset as usize;
        if index < self.tokens.len() {
            self.tokens[index].clone()
        } else {
            Token::eof()
        }
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    fn match_type(&mut self, kind: TokenType) -> bool {
        let current = self.get(0);
        if kind == current.kind {
            self.next();
            true
        } else {
            false
        }
    }

    fn expression(&mut self) -> ExprT {
        return self.additive();
    }

    fn additive(&mut self) -> ExprT {
        let expr = self.multiplicative();
        loop {
            if self.match_type(TT::Plus) {
                let right = self.multiplicative();
                return Box::new(BinExpr::new(expr, Op::Add, right));
            } else if self.match_type(TT::Minus) {
                let right = self.multiplicative();
                return Box::new(BinExpr::new(expr, Op::Sub, right));
            }
            break;
        }
        expr
    }

    fn multiplicative(&mut self) -> ExprT {
        let expr = self.unary();
        loop {
            if self.match_type(TT::Star) {
                let right = self.unary();
                return Box::new(BinExpr::new(expr, Op::Mul, right));
            } else if self.match_type(TT::Slash) {
                let right = self.unary();
                return Box::new(BinExpr::new(expr, Op::Div, right));
            } else {
                break;
            }
        }
        expr
    }

    fn unary(&mut self) -> ExprT {
        let token = self.get(0);
        match token.kind {
            TT::Minus => {
                self.next();
                Box::new(UnaryExpr::new(self.primary(), Unary::Negative))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> ExprT {
        let token = self.get(0);
        match token.kind {
            TT::Number(n) => {
                self.next();
                let expr = NumExpr::new(n);
                Box::new(expr)
            }
            _ => panic!("Unknown expression in {}:{}", token.line, token.column),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<ExprT> {
    let exprs = Vec::new();
    let mut pr = Parser::new(tokens, exprs);
    while !pr.match_type(TT::Eof) {
        let expr = pr.expression();
        pr.exprs.push(expr);
    }
    pr.exprs
}
