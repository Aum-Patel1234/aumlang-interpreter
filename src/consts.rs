use crate::token::Keyword;

pub const PRINT_KEYWORD: &str = "print";
pub const LET_KEYWORD: &str = "let";
pub const NULL_KEYWORD: &str = "null";
pub const FUNCTION_KEYWORD: &str = "fn";
pub const IF_KEYWORD: &str = "if";
pub const ELSE_KEYWORD: &str = "else";
pub const FOR_KEYWORD: &str = "for";
pub const WHILE_KEYWORD: &str = "while";
pub const AND_KEYWORD: &str = "and";
pub const OR_KEYWORD: &str = "or";
pub const RETURN_KEYWORD: &str = "return";
pub const TRUE_KEYWORD: &str = "true";
pub const FALSE_KEYWORD: &str = "false";

pub fn look_keyword(identifier: &str) -> Option<Keyword> {
    match identifier {
        LET_KEYWORD => Some(Keyword::LET),
        PRINT_KEYWORD => Some(Keyword::PRINT),
        FUNCTION_KEYWORD => Some(Keyword::FUNCTION),
        IF_KEYWORD => Some(Keyword::IF),
        ELSE_KEYWORD => Some(Keyword::ELSE),
        FOR_KEYWORD => Some(Keyword::FOR),
        WHILE_KEYWORD => Some(Keyword::WHILE),
        AND_KEYWORD => Some(Keyword::AND),
        OR_KEYWORD => Some(Keyword::OR),
        RETURN_KEYWORD => Some(Keyword::RETURN),
        NULL_KEYWORD => Some(Keyword::NULL),
        TRUE_KEYWORD => Some(Keyword::TRUE),
        FALSE_KEYWORD => Some(Keyword::FALSE),
        _ => None,
    }
}
