use core::str;

use crate::token::{Keyword, Operator, Token};

pub fn get_tokens(line: &str) -> Vec<Token> {
    let words: Vec<&str> = line.split(" ").collect();
    let n = words.len();
    let mut tokens: Vec<Token> = Vec::with_capacity(n);

    for word in words {
        match word {
            "print" => tokens.push(Token::Keyword(Keyword::Print)),
            "+" => tokens.push(Token::Operator(Operator::Plus)),
            "-" => tokens.push(Token::Operator(Operator::Minus)),
            "*" => tokens.push(Token::Operator(Operator::Star)),
            "/" => tokens.push(Token::Operator(Operator::Slash)),
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "{" => tokens.push(Token::LBrace),
            "}" => tokens.push(Token::RBrace),
            _ => {}
        }
    }

    tokens
}

pub fn print_tokens(tokens: &[Token]) {
    tokens.iter().for_each(|t| println!("{:?}", t));
}
