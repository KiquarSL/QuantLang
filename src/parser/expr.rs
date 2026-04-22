use crate::runtime::{Runtime, Value};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Var(String),
    Bool(bool),
    Bin(Box<Expr>, Op, Box<Expr>),
    Unary(Box<Expr>, Unary),
}

impl Expr {
    pub fn eval(&self, rt: &mut Runtime) -> Value {
        match self {
            Expr::Number(v) => Value::F64(*v),
            Expr::String(v) => Value::String(v.clone()),
            Expr::Bool(v) => Value::Bool(*v),
            Expr::Var(name) => rt
                .get_var(name.to_string())
                .unwrap_or_else(|| panic!("Variable '{}' not found", name))
                .clone(),
            Expr::Unary(inner, unary) => {
                let val = inner.eval(rt);
                match unary {
                    Unary::Positive => val,
                    Unary::Negative => match val {
                        Value::F64(n) => Value::F64(-n),
                        _ => panic!("Negative only for numbers"),
                    },
                }
            }
            Expr::Bin(left, op, right) => {
                let l = left.eval(rt);
                let r = right.eval(rt);
                match (l, r) {
                    (Value::F64(a), Value::F64(b)) => Value::F64(match op {
                        Op::Add => a + b,
                        Op::Sub => a - b,
                        Op::Mul => a * b,
                        Op::Div => a / b,
                    }),
                    (Value::String(a), Value::String(b)) => Value::String(match op {
                        Op::Add => a + &b,
                        _ => panic!("String only supports + operator"),
                    }),
                    _ => panic!("Type mismatch in binary operation"),
                }
            }
        }
    }
}

// Конструкторы
impl Expr {
    pub fn number(n: f64) -> Self {
        Expr::Number(n)
    }

    pub fn string(s: &str) -> Self {
        Expr::String(s.to_string())
    }

    pub fn var(name: &str) -> Self {
        Expr::Var(name.to_string())
    }

    pub fn bool(b: bool) -> Self {
        Expr::Bool(b)
    }

    pub fn bin(left: Expr, op: Op, right: Expr) -> Self {
        Expr::Bin(Box::new(left), op, Box::new(right))
    }

    pub fn unary(inner: Expr, unary: Unary) -> Self {
        Expr::Unary(Box::new(inner), unary)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Bin(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Unary(inner, u) => match u {
                Unary::Positive => write!(f, "(+{})", inner),
                Unary::Negative => write!(f, "(-{})", inner),
            },
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unary {
    Positive,
    Negative,
}
