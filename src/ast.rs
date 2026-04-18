use crate::{
    token::{Operator, Token, Value},
    utils::print_error,
};

// enum Expr {
//     Value(Token),
//     Binary {
//         left: Box<Expr>,
//         op: Operator,
//         right: Box<Expr>,
//     },
// }

// fn make_expr(token: Token) -> Box<Expr> {
//     Box::new(Expr::Value(token))
// }
//
// fn make_binary_expr(left: Box<Expr>, op: Operator, right: Box<Expr>) -> Box<Expr> {
//     Box::new(Expr::Binary {
//         left: left,
//         op: op,
//         right: right,
//     })
// }

pub fn evaluate_expr(tokens: &[Token]) -> Value {
    // let mut stack: Vec<Expr> = Vec::new();
    let mut iter = tokens.iter().peekable();

    // println!("Before skip: {:?}", tokens);

    if matches!(iter.peek(), Some(Token::Identifier(_))) {
        iter.next();
    }
    if !matches!(iter.peek(), Some(Token::Operator(Operator::Equal))) {
        // || // TODO:!map.present(tokens.first())
        match tokens.first() {
            Some(Token::Identifier(name)) => {
                print_error(&format!(
                    "Error in syntax, expected '=' after variable({})",
                    name
                ));
            }
            _ => {
                eprintln!("Error: expected identifier at start");
            }
        };
    }

    // for token in iter {
    // println!("Processing token: {:?}", token);
    // }

    Value::Double(0.0)
}
