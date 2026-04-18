use core::str;

use crate::token::{Keyword, Operator, Token, Value};

pub fn get_tokens(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();

    while i < n {
        let ch = chars[i];

        match ch {
            ' ' | '\t' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Operator(Operator::Plus));
                i += 1;
            }
            '-' => {
                tokens.push(Token::Operator(Operator::Minus));
                i += 1;
            }
            '/' => {
                tokens.push(Token::Operator(Operator::Slash));
                i += 1;
            }
            '*' => {
                tokens.push(Token::Operator(Operator::Star));
                i += 1;
            }
            '=' => {
                tokens.push(Token::Operator(Operator::Equal));
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                i += 1;
            }

            '0'..='9' => {
                let mut dot_seen = false;
                let mut num = String::new();

                // HACK: here i did dot_seen cause to avoid 24.2.4
                while i < n && (chars[i].is_ascii_digit() || (!dot_seen && chars[i] == '.')) {
                    if chars[i] == '.' {
                        dot_seen = true;
                    }
                    num.push(chars[i]);
                    i += 1;
                }

                // NOTE: i am only converting to Double nomatter what number
                // main thing is logic not so much of specifics
                tokens.push(Token::Value(Value::Double(num.parse().unwrap())));
            }

            // TODO: support string value type with start of  ""
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut variable = String::new();

                while i < n && (chars[i].is_alphabetic() || chars[i] == '_') {
                    variable.push(chars[i]);
                    i += 1;
                }

                match variable.as_str() {
                    "print" => tokens.push(Token::Keyword(Keyword::Print)),
                    // TODO:
                    // "if" => tokens.push(Token::Keyword(Keyword::IF)),
                    // "else" => tokens.push(Token::Keyword(Keyword::ELSE)),
                    // "while" => tokens.push(Token::Keyword(Keyword::WHILE)),
                    // "and" => tokens.push(Token::Keyword(Keyword::AND)),
                    // "for" => tokens.push(Token::Keyword(Keyword::FOR)),
                    // "or" => tokens.push(Token::Keyword(Keyword::OR)),
                    // "return" => tokens.push(Token::Keyword(Keyword::RETURN)),
                    "null" => tokens.push(Token::Value(Value::NULL)),
                    _ => tokens.push(Token::Identifier(variable)),
                }
            }

            _ => {
                println!("Unknown character: {}", ch);
                i += 1;
            }
        }
    }

    tokens
}

pub fn print_tokens(tokens: &[Token]) {
    print!("[");
    tokens.iter().for_each(|t| print!("{}, ", t));
    println!("]");
}
