use std::fmt;

type ExprT = Box<dyn Expr>;

pub trait Expr {
    fn eval(&self) -> f64;
}

pub struct NumExpr {
    num: f64,
}

impl NumExpr {
    pub fn new(num: f64) -> Self {
        Self { num }
    }
}

impl fmt::Display for ExprT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.eval())
    }
}

impl Expr for NumExpr {
    fn eval(&self) -> f64 {
        self.num
    }
}

pub struct UnaryExpr {
    num: ExprT,
    unary: Unary,
}

impl UnaryExpr {
    pub fn new(num: ExprT, unary: Unary) -> Self {
        Self { num, unary }
    }
}

impl Expr for UnaryExpr {
    fn eval(&self) -> f64 {
        match self.unary {
            Unary::Positive => self.num.eval(),
            Unary::Negative => -self.num.eval(),
        }
    }
}

pub struct BinExpr {
    num1: ExprT,
    num2: ExprT,
    op: Op,
}

impl BinExpr {
    pub fn new(num1: ExprT, op: Op, num2: ExprT) -> Self {
        Self { num1, num2, op }
    }
}

impl Expr for BinExpr {
    fn eval(&self) -> f64 {
        match self.op {
            Op::Add => self.num1.eval() + self.num2.eval(),
            Op::Sub => self.num1.eval() - self.num2.eval(),
            Op::Mul => self.num1.eval() * self.num2.eval(),
            Op::Div => self.num1.eval() / self.num2.eval(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Unary {
    Positive,
    Negative,
}
