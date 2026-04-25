use std::ops::Deref;

use crate::token::{Keyword, Token};

// Traits
pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

// impls for Program
impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            return "";
        }
        // TODO:
        self.statements[0].token_literal()
    }
}

// LetStatemet
pub struct LetStatemet {
    keyword: Keyword,
    name: Identifier,
    value: Box<dyn Expression>,
}
// impls for LetStatemet
impl LetStatemet {
    pub fn new(keyword: Keyword, name: Identifier, value: Box<dyn Expression>) -> LetStatemet {
        LetStatemet {
            keyword,
            name,
            value,
        }
    }

    // TODO: remove this fn when implemented statements
    pub fn read(&self) {
        println!("{}", self.keyword);
        let b = &self.name;
        println!("{}", b.value);
        let a = self.value.deref();
        a.expression_node();
    }
}
impl Node for LetStatemet {
    fn token_literal(&self) -> &str {
        self.keyword.as_str()
    }
}
impl Statement for LetStatemet {
    fn statement_node(&self) {}
}

// Identifier
pub struct Identifier {
    value: String,
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
impl Expression for Identifier {
    fn expression_node(&self) {}
}
