mod types;
mod lexer;
use crate::{lexer::lex};


fn main() {
    let source :&str = "print(34);";
    println!("{:?}", lex(source));
}
