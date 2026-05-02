use crate::parser::{ast::Expression, parser_logic::Parser};

pub type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Option<Expression>;
pub type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Option<Expression>;
