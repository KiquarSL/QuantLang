use quant::lexer::tokenize;
use quant::parser::parse;
use quant::runtime::Runtime;
use std::fs;

#[test]
fn ast() {
    let test = fs::read_to_string("tests/test.qu").unwrap();

    println!("\n{}", "\x1b[35mTesting AST\x1b[0m");
    let tokens = tokenize(test);
    let stmts = parse(tokens);
    let rt = &mut Runtime::new();
    for stmt in stmts {
        stmt.exec(rt);
    }
    println!("\n\x1b[35mRuntime vars: \x1b[0m {:?}", rt.vars);
}
