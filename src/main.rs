mod lexer;
use lexer::tokenize;

fn main() {
    let test = "1 + 2".to_string();

    let tokens = tokenize(test);
    for i in tokens {
        println!("{}", i);
    }
}
