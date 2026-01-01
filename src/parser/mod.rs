use crate::types::{Delimiter, Keyword, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.pos + 1]
    }
    fn next(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok
    }
    pub fn parse(&self) {
        
    }
}

// pub fn parse(tokens: Vec<Token>) {
//     let mut peekable_tokens = tokens.iter().peekable();
//     while let Some(&token) = peekable_tokens.peek() {
//         match token {
//             Token::Keyword(Keyword::Print) => {
//                 peekable_tokens.next();
//                 if peekable_tokens.peek() == Token::Delimiter(Delimiter::LeftParen){
//                     peekable_tokens.next();
//                 }
//             }
//             _ => {
//                 println!("other");
//                 peekable_tokens.next();
//             }
//         }
//     }
// }
