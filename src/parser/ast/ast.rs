use crate::parser::Expr;
use crate::runtime::Runtime;

pub trait Stmt {
    fn exec(&self, rt: &mut Runtime);
}

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
