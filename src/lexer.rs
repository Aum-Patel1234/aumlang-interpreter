use core::str;

use crate::{
    consts::look_keyword,
    token::{Operator, Token, Value},
};

const COMMENT_PREFIX: char = '#';

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new_lexer(input_str: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input_str,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input.as_bytes()[self.read_position] as char
    }
    fn skip_extras(&mut self) {
        loop {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }

            if self.ch == COMMENT_PREFIX {
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_extras();

        // println!("Current char: {:?}", self.ch);
        let token = match self.ch {
            // ' ' | '\t' => {
            //     i += 1;
            // }
            '+' => Token::Operator(Operator::Plus),
            '-' => Token::Operator(Operator::Minus),
            '/' => Token::Operator(Operator::Slash),
            '*' => Token::Operator(Operator::Star),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::EQ)
                } else {
                    Token::Operator(Operator::Equal)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::NEQ)
                } else {
                    Token::Operator(Operator::Exclamation)
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::LTE)
                } else {
                    Token::Operator(Operator::LT)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::GTE)
                } else {
                    Token::Operator(Operator::GT)
                }
            }
            '%' => Token::Operator(Operator::Modulo),
            '&' => Token::Operator(Operator::BitwiseAnd),
            '|' => Token::Operator(Operator::BitwiseOr),
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,

            ',' => Token::Comma,
            ';' => Token::Semicolon,

            '"' => Token::Operator(Operator::DoubleQuote),

            '\0' => Token::EOF,
            _ => {
                if Lexer::is_letter(self.ch) {
                    return self.read_identifier();
                } else if Lexer::is_number(self.ch) {
                    return self.read_literal();
                }

                Token::Unknown
            }
        };
        self.read_char();
        token
    }

    pub fn read_literal(&mut self) -> Token {
        let pos = self.position;
        while Lexer::is_number(self.ch) {
            self.read_char();
        }

        let token_string = &self.input[pos..self.position];
        // TODO: Handle string literals
        match token_string.parse::<f64>() {
            Ok(v) => Token::Value(Value::Double(v)),
            Err(_) => Token::Unknown,
        }
    }

    pub fn read_identifier(&mut self) -> Token {
        let pos = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        let token_string = &self.input[pos..self.position];
        match look_keyword(token_string) {
            Some(keyword) => Token::Keyword(keyword),
            None => Token::Identifier(token_string.to_string()),
        }
    }

    fn is_letter(ch: char) -> bool {
        // ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
        ch.is_alphabetic() || ch == '_'
    }
    fn is_number(ch: char) -> bool {
        ch.is_ascii_digit()
    }
}

pub fn print_tokens(tokens: &[Token]) {
    print!("[");
    tokens.iter().for_each(|t| print!("{}, ", t));
    println!("]");
}
