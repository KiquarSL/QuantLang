use crate::parser::Expr;
use crate::runtime::Runtime;

pub trait Stmt {
    fn exec(&self, rt: &mut Runtime);
}

/* Block stmt */
pub struct Block {
    stmts: Vec<Box<dyn Stmt>>,
}

impl Block {
    pub fn new(stmts: Vec<Box<dyn Stmt>>) -> Self {
        Self { stmts }
    }
}

impl Stmt for Block {
    fn exec(&self, rt: &mut Runtime) {
        for stmt in &self.stmts {
            stmt.exec(rt);
        }
    }
}

/* New Var Statement */
pub struct NewVarStmt {
    id: String,
    value: Expr,
}

impl NewVarStmt {
    pub fn new(id: String, value: Expr) -> Self {
        Self { id, value }
    }
}

impl Stmt for NewVarStmt {
    fn exec(&self, rt: &mut Runtime) {
        let val = self.value.eval(rt);
        rt.new_var(self.id.clone(), val);
    }
}

/* If-Else Statement*/

pub struct IfStmt {
    pub cond: Expr,
    pub then_b: Block,
    pub else_b: Option<Block>,
}

impl IfStmt {
    pub fn new(cond: Expr, then_b: Block, else_b: Option<Block>) -> Self {
        Self {
            cond,
            then_b,
            else_b,
        }
    }
}

impl Stmt for IfStmt {
    fn exec(&self, rt: &mut Runtime) {
        let val = self.cond.eval(rt);
        if val.is_truth() {
            self.then_b.exec(rt);
        } else if let Some(block) = &self.else_b {
            block.exec(rt);
        }
    }
}

/* assign Statement*/

pub struct AssignStmt {
    id: String,
    val: Expr,
}

impl AssignStmt {
    pub fn new(id: String, val: Expr) -> Self {
        Self { id, val }
    }
}

impl Stmt for AssignStmt {
    fn exec(&self, rt: &mut Runtime) {
        let val = self.val.eval(rt);
        rt.set_var(self.id.clone(), val);
    }
}
