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
                match (l.clone(), r.clone()) {
                    (Value::F64(a), Value::F64(b)) => match op {
                        Op::Add => Value::F64(a + b),
                        Op::Sub => Value::F64(a - b),
                        Op::Mul => Value::F64(a * b),
                        Op::Div => Value::F64(a / b),
                        Op::Eq => Value::Bool(a == b),
                        Op::Ne => Value::Bool(a != b),
                        Op::Lt => Value::Bool(a < b),
                        Op::Gt => Value::Bool(a > b),
                        Op::Le => Value::Bool(a <= b),
                        Op::Ge => Value::Bool(a >= b),
                    },

                    (Value::String(a), Value::String(b)) => match op {
                        Op::Add => Value::String(a + &b),
                        Op::Eq => Value::Bool(a == b),
                        Op::Ne => Value::Bool(a != b),
                        _ => panic!("String only supports +, ==, !="),
                    },

                    (Value::Bool(a), Value::Bool(b)) => match op {
                        Op::Eq => Value::Bool(a == b),
                        Op::Ne => Value::Bool(a != b),
                        _ => panic!("Bool only supports ==, !="),
                    },
                    _ => panic!(
                        "Type mismatch in binary operation: {:?} vs {:?}",
                        l.clone(),
                        r.clone()
                    ),
                }
            }
        }
    }
}

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
            Op::Eq => write!(f, "=="),
            Op::Ne => write!(f, "!="),
            Op::Lt => write!(f, "<"),
            Op::Gt => write!(f, ">"),
            Op::Le => write!(f, "<="),
            Op::Ge => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq, // ==
    Ne, // !=
    Lt, // <
    Gt, // >
    Le, // <=
    Ge, // >=
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unary {
    Positive,
    Negative,
}
