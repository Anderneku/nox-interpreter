use crate::types::*;
pub struct Lexer {
    source: String,
}
impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }
    pub fn lex(&mut self) -> Vec<Token>{
        let mut tokens: Vec<Result<Token, Error>> = Vec::new();
        let mut column = 1;
        let mut line = 1;
        let mut chars = self.source.chars().peekable();

        while let Some(&ch) = chars.peek() {
            if ch == '\n' {
                line += 1;
                column = 1;
                chars.next();
            } else if ch.is_whitespace() {
                chars.next();
                column += 1;
            } else if ch.is_ascii_alphabetic() {
                let mut buffer = String::new();
                while let Some(&alphabet) = chars.peek() {
                    if alphabet.is_alphanumeric() || alphabet == '_' {
                        buffer.push(alphabet);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                // Keyword match
                let token = match buffer.as_str() {
                    "print" => Token::Identifier(Identifier::String(String::from("print"))),
                    "let" => Token::Keyword(Keyword::Let),
                    // Literal match
                    "true" => Token::Literal(Literal::Boolean(true)),
                    "false" => Token::Literal(Literal::Boolean(false)),
                    _ => {
                        // Try number literal
                        if let Ok(n) = buffer.parse::<i32>() {
                            Token::Literal(Literal::Number(n))
                        } else {
                            Token::Identifier(Identifier::String(buffer))
                        }
                    }
                };

                tokens.push(Ok(token));
            } else if ch.is_ascii_digit() {
                let mut buffer = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        buffer.push(digit);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                if let Ok(n) = buffer.parse::<i32>() {
                    tokens.push(Ok(Token::Literal(Literal::Number(n))));
                } else {
                    tokens.push(Err(Error {
                        error_type: ErrorTypes::Lex(LexError::UnknownToken),
                        column: column,
                        line_number: line,
                    }));
                }
            } else if let Some(op) = match ch {
                '+' => Some(Operator::Add),
                '-' => Some(Operator::Subtract),
                '*' => Some(Operator::Multiply),
                '/' => Some(Operator::Divide),
                _ => None,
            } {
                tokens.push(Ok(Token::Operator(op)));
                chars.next();
                column += 1;
            } else if let Some(dl) = match ch {
                '(' => Some(Delimiter::LeftParen),
                ')' => Some(Delimiter::RightParen),
                ';' => Some(Delimiter::Semicolon),
                ',' => Some(Delimiter::Comma),
                _ => None,
            } {
                tokens.push(Ok(Token::Delimiter(dl)));
                chars.next();
                column += 1;
            } else {
                tokens.push(Err(Error {
                    error_type: ErrorTypes::Lex(LexError::UnknownToken),
                    column: column,
                    line_number: line,
                }));
                chars.next();
                column += 1;
            }
        }

        check_errors(&tokens);

        let mut new_tokens: Vec<Token> = Vec::new();
        for token in tokens {
            new_tokens.push(token.unwrap());
        }
        new_tokens
    }
}

fn check_errors(tokens: &Vec<Result<Token, Error>>) {
    let mut error_found = false;
    for item in tokens.iter() {
        match item {
            Ok(_) => {}
            Err(e) => {
                error_found = true;
                println!("{}", e);
            }
        }
    }
    if error_found {
        std::process::exit(0);
    }
}
