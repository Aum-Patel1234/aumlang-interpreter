use core::fmt;

#[derive(Debug)]
pub enum Keyword {
    Print,
    // IF,
    // ELSE,
    // FOR,
    // AND,
    // OR,
    // TRUE,
    // FALSE,
    // RETURN,
    // NULL,  // see if we want a Option type
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    // Xor,
    // Caret, // for power make an inbuilt function
    // BitwiseAnd,
    // BitwiseOr,
    // Modulo,
    // Exclamation,
    // Equality, // ==
    // NotEqual, // !=
    // GreaterThanEqualTo // >=
    // LessThanEqualTo // <=
}

#[derive(Debug)]
pub enum Value {
    // Int(i32),
    Double(f64),
    StringLiteral(String),
    // Char(char),
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
    Value(Value),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "Keyword::{:?}", k),
            Token::Identifier(name) => write!(f, "Identifier(Name) = {}", name),

            Token::Operator(op) => write!(f, "Operator::{:?}", op),

            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),

            Token::Value(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Token::Int(v) => write!(f, "Int({})", v),
            Value::Double(v) => write!(f, "Value::Double({})", v),
            Value::StringLiteral(s) => write!(f, "Value::String(\"{}\")", s),
            // Value::Char(c) => write!(f, "Char('{}')", c),
        }
    }
}
