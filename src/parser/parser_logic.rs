use crate::{lexer::Lexer, parser::ast::Program, token::Token};

pub struct Parser<'a> {
    l: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::Unknown,
            peek_token: Token::Unknown,
        };
        p.next_token();
        p.next_token();

        p
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program() -> Option<Program> {
        // TODO:
        None
    }
}
