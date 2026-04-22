use quant::lexer::tokenize;
use std::fs;

#[test]
fn lexer() {
    let test = fs::read_to_string("tests/test.qu").unwrap();
    println!("\x1b[36mExample\x1b[0m: {}", test);

    println!("\n{}", "\x1b[35mTesting lexer\x1b[0m");
    let tokens = tokenize(test);
    for i in tokens.clone() {
        println!("{}", i);
    }
}
