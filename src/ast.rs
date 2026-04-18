use crate::token::{Operator, Token};

enum Expr {
    Value(Token),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}
