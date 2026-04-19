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
//
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

pub fn evaluate_expr(tokens: &[Token]) -> Option<Value> {
    // let mut stack: Vec<Expr> = Vec::new();

    match tokens.first() {
        Some(Token::Identifier(_)) => {
            let mut iter = tokens.iter().peekable();

            if !is_valid_expr(&mut iter) {
                return None;
            }

            // TODO: evaluate using ast
            // for token in iter {}
        }
        Some(Token::Value(_)) => {}
        None => {}
        _ => {}
    }

    // for token in iter {
    // println!("Processing token: {:?}", token);
    // }

    None
}

fn is_valid_expr(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> bool {
    // println!("Before skip: {:?}", tokens);
    let first = match iter.peek() {
        Some(Token::Identifier(name)) => {
            iter.next();
            name
        }
        _ => {
            eprintln!("Error: expected identifier at start");
            return false;
        }
    };

    if !matches!(iter.peek(), Some(Token::Operator(Operator::Equal))) {
        // || // TODO:!map.present(tokens.first())
        print_error(&format!(
            "Error in syntax, expected '=' after variable({})",
            first
        ));
        return false;
    }

    // skip: a =
    iter.next();

    true
}
