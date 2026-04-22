#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I32(i32),
    F64(f64),
    Char(char),
    Bool(bool),
    String(String),
    Null,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::I32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Char(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Null => "null".to_string(),
        }
    }

    pub fn is_truth(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            _ => false,
        }
    }
}
