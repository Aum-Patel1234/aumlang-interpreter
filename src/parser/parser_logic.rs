use crate::{
    lexer::Lexer,
    parser::ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::{Keyword, Operator, Token},
};

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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            let statement: Result<Statement, String> = match &self.curr_token {
                Token::Keyword(kw) => match kw {
                    Keyword::LET => self.parse_let_statement(),
                    Keyword::RETURN => self.parse_return_statement(),
                    Keyword::IF => self.parse_if_statement(),
                    _ => Err("Invalid statement".to_string()),
                },
                _ => Err("Wrong statement".to_string()),
            };

            if let Ok(stmt) = statement {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        // Eg: let <identifier> = <expression>;
        self.next_token(); // skip the let token

        let identifier = Identifier::new(self.curr_token.clone())?;
        self.next_token(); // skip identifier

        if self.curr_token != Token::Operator(Operator::Equal) {
            return Err(String::from(
                "Expected '=' after identifier in LET statement.",
            ));
        }
        self.next_token(); // skip =

        let expression = self.parse_expression();

        Ok(Statement::Let(LetStatement::new(
            Keyword::LET,
            identifier,
            expression,
        )))
    }
    fn parse_return_statement(&self) -> Result<Statement, String> {
        Err(String::from("Not impelmented"))
    }
    fn parse_if_statement(&self) -> Result<Statement, String> {
        Err(String::from("Not impelmented"))
    }

    fn parse_expression(&self) -> Expression {
        Expression::Identifier(Identifier {
            value: String::from("Not impelmented"),
        })
    }
}
