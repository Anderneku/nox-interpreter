mod types;
mod lexer;
use crate::{lexer::lex, types::*};


fn main() {
    let source :&str = "let print(34);";
    println!("{:?}", lex(source));
}
