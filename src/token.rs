use core::fmt;

#[derive(Debug)]
pub enum Keyword {
    Print,
    // IF,
    // ELSE,
    // FOR
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Operator(Operator),
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    // LBracket, // [
    // RBracket, // ]

    // Values
    Int(i32),
    Double(f64),
    StringLiteral(String),
    Char(char),
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Keyword::Print => "Keyword(print)",
        };
        write!(f, "{}", symbol)
    }
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Equal => "=",
        };
        write!(f, "{}", symbol)
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "Keyword::{:?}", k),
            Token::Identifier(name) => write!(f, "Identifier({})", name),

            Token::Operator(op) => write!(f, "Operator::{:?}", op),

            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),

            Token::Int(v) => write!(f, "Int({})", v),
            Token::Double(v) => write!(f, "Double({})", v),
            Token::StringLiteral(s) => write!(f, "String(\"{}\")", s),
            Token::Char(c) => write!(f, "Char('{}')", c),
        }
    }
}
