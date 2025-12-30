
use std::{fmt::Display};

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Delimiter(Delimiter),
}

#[derive(Debug)]
pub enum Keyword {
    Print,
    Let,
}

#[derive(Debug)]
pub enum Literal {
    Number(i32),
    String(String),
    Boolean(bool),
}

#[derive(Clone, Copy, Debug)]
pub enum Delimiter {
    LeftParen,
    RightParen,
    Semicolon,
    Comma,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Operator {
    pub fn as_char(self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Subtract => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub errorType: ErrorTypes,
    pub column: u32,
    pub line_number: u32,
}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: Unknown Token at line: {}, column: {}", self.errorType, self.line_number, self.column)
    }
}

#[derive(Debug)]
pub enum ErrorTypes {
    Lex(LexError),
}

#[derive(Debug)]
pub enum LexError {
    UnknownToken,
}

