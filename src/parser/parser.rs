use crate::lexer::{Token, TokenType};
use crate::parser::{
    Block, Stmt, is_assign_stmt, is_if_stmt, is_new_var_stmt, parse_assign_stmt, parse_if_stmt,
    parse_new_var_stmt,
};
use crate::parser::{Expr, Op, Unary};

type TT = TokenType;
type StmtT = Box<dyn Stmt>;

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    stmts: Vec<StmtT>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            pos: 0,
            tokens,
            stmts: Vec::new(),
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
        if current.kind == kind {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) {
        while self.get(0).kind != TT::Eof {
            let stmt = self.parse_stmt();
            self.stmts.push(stmt);
        }
    }

    pub fn parse_block(&mut self) -> Block {
        let mut stmts = Vec::new();
        self.match_type(TT::LBracket); // {
        while !self.match_type(TT::RBracket) && self.get(0).kind != TT::Eof {
            stmts.push(self.parse_stmt());
        }
        Block::new(stmts)
    }

    pub fn parse_stmt(&mut self) -> StmtT {
        if is_new_var_stmt(self) {
            parse_new_var_stmt(self)
        } else if is_assign_stmt(self) {
            parse_assign_stmt(self)
        } else if is_if_stmt(self) {
            parse_if_stmt(self)
        } else {
            let token = self.get(0);
            panic!("Unknown statement at {}:{}", token.line, token.column);
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.comparison()
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.additive();
        loop {
            let op = match self.get(0).kind {
                TT::Eq => Op::Eq,
                TT::Ne => Op::Ne,
                TT::Ge => Op::Ge,
                TT::Le => Op::Le,
                TT::Gt => Op::Gt,
                TT::Lt => Op::Lt,
                _ => break,
            };
            self.next();
            let right = self.additive();
            left = Expr::bin(left, op, right);
        }
        left
    }

    fn additive(&mut self) -> Expr {
        let mut left = self.multiplicative();
        loop {
            let op = match self.get(0).kind {
                TT::Plus => Op::Add,
                TT::Minus => Op::Sub,
                _ => break,
            };
            self.next();
            let right = self.multiplicative();
            left = Expr::bin(left, op, right);
        }
        left
    }

    fn multiplicative(&mut self) -> Expr {
        let mut left = self.unary();
        loop {
            let op = match self.get(0).kind {
                TT::Star => Op::Mul,
                TT::Slash => Op::Div,
                _ => break,
            };
            self.next();
            let right = self.unary();
            left = Expr::bin(left, op, right);
        }
        left
    }

    fn unary(&mut self) -> Expr {
        let token = self.get(0);
        match token.kind {
            TT::Minus => {
                self.next();
                Expr::unary(self.primary(), Unary::Negative)
            }
            TT::Plus => {
                self.next();
                self.unary()
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.get(0);
        match token.kind {
            TT::Number(n) => {
                self.next();
                Expr::number(n)
            }
            TT::Id(name) => {
                self.next();
                Expr::var(&name)
            }
            TT::String(s) => {
                self.next();
                Expr::string(&s)
            }
            TT::True => {
                self.next();
                Expr::bool(true)
            }
            TT::False => {
                self.next();
                Expr::bool(false)
            }
            TT::LParent => {
                self.next();
                let expr = self.expression();
                self.match_type(TT::RParent);
                expr
            }
            _ => panic!("Unexpected token at {}:{}", token.line, token.column),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<StmtT> {
    let mut pr = Parser::new(tokens);
    pr.parse();
    pr.stmts
}
