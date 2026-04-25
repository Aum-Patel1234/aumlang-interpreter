use crate::token::{Keyword, Token};

// Traits
pub trait Node {
    fn token_literal(&self) -> &str;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    // return, expression
}
impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(s) => s.token_literal(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    // TODO: IntegerLiteral, PrefixExpr, etc.
}
impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
        }
    }
}

// Program
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }
}
impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            return "";
        }
        // TODO:
        self.statements[0].token_literal()
    }
}

// LetStatement
#[derive(Debug)]
pub struct LetStatement {
    pub keyword: Keyword,
    pub name: Identifier,
    pub value: Expression,
}
// impls for LetStatement
impl LetStatement {
    pub fn new(keyword: Keyword, name: Identifier, value: Expression) -> LetStatement {
        LetStatement {
            keyword,
            name,
            value,
        }
    }

    // TODO: remove this fn when implemented statements
    pub fn read(&self) {
        println!("{}", self.keyword);
        let b = &self.name;
        println!("{} {}", b.value, self.value.token_literal());
    }
}
impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.keyword.as_str()
    }
}

// Identifier
#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

// impls for Identifier
impl Identifier {
    pub fn new(token: Token) -> Result<Identifier, String> {
        match token {
            Token::Identifier(val) => Ok(Identifier { value: val }),
            _ => Err(format!(
                "Error while Identifier::new() --> Expected Token::Identifier, got {:?}",
                token
            )),
        }
    }
}
impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.value
    }
}
