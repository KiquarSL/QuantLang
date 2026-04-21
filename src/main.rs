mod lexer;
mod parser;
use lexer::tokenize;
use parser::parse;

fn main() {
    let test = "(2 + 2) * -2".to_string();
    println!("{}: {}", "\x1b[36mExample\x1b[0m", test);

    println!("\n{}", "\x1b[35mTesting lexer\x1b[0m");
    let tokens = tokenize(test);
    for i in tokens.clone() {
        println!("{}", i);
    }

    println!("\n{}\n", "\x1b[35mTesting parser\x1b[0m");
    let exprs = parse(tokens.clone());
    for i in exprs {
        println!("{}", i);
    }
}
