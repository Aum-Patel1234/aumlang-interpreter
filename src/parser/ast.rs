use crate::token::{Keyword, Token};

// Traits
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    // return, expression
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal().to_string(),
            Statement::Return(s) => s.token_literal().to_string(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Statement::Let(let_statement) => let_statement.string(),
            Statement::Return(return_statement) => return_statement.string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    // TODO: IntegerLiteral, PrefixExpr, etc.
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal().to_string(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Expression::Identifier(identifier) => identifier.string(),
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
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return "".to_string();
        }
        // TODO:
        self.statements[0].token_literal()
    }

    fn string(&self) -> String {
        let mut string = String::new();

        for stmt in &self.statements {
            let mut out = stmt.string();
            out.push('\n');
            string.push_str(&out);
        }

        string
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
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.keyword.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push(' ');
        out.push_str(self.name.string().as_str());
        out.push_str(" = ");

        out.push_str(self.value.string().as_str());
        out.push(';');

        out
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
    fn token_literal(&self) -> String {
        self.value.to_string()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

// ReturnStatement
#[derive(Debug)]
pub struct ReturnStatement {
    pub keyword: Keyword,
    pub return_val: Expression,
}
impl ReturnStatement {
    pub fn new(keyword: Keyword, return_val: Expression) -> ReturnStatement {
        ReturnStatement {
            keyword,
            return_val,
        }
    }
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.keyword.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&(self.token_literal() + " "));
        match &self.return_val {
            Expression::Identifier(identifier) => out.push_str(identifier.string().as_str()),
        }
        out.push(';');

        out
    }
}

pub struct ExpressionStatement {
    token: Token,
    expression: Expression,
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Expression) -> ExpressionStatement {
        ExpressionStatement { token, expression }
    }
    pub fn read(&self) {
        println!("{}", self.expression.token_literal());
    }
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        match &self.expression {
            Expression::Identifier(identifier) => identifier.string(),
        }
    }
}
