mod lexer;
mod parser;
use lexer::tokenize;
use parser::parse;

fn main() {
    println!("{}", "\\e[36mTesting lexere[0m");
    let test = "1 + 2".to_string();
    let tokens = tokenize(test);
    for i in tokens.clone() {
        println!("{}", i);
    }

    println!("{}", "\\e[36mTesting parser\\e[0m");
    let exprs = parse(tokens.clone());
    for i in exprs {
        println!("{}", i);
    }
}
