use crate::parser::{ast::Expression, parser_logic::Parser};

pub type PrefixParseFn = fn(&mut Parser) -> Expression;
pub type InfixParseFn = fn(&mut Parser, Expression) -> Expression;
