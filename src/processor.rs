use crate::{
    ast::evaluate_expr,
    lexer::{get_tokens, print_tokens},
    token::{Operator, Token},
    utils::print_error,
};

pub fn process_line(line: &str) {
    if line.is_empty() || line.starts_with("#") {
        return;
    }

    let tokens: Vec<Token> = get_tokens(line);

    match tokens.first() {
        Some(Token::Keyword(_)) => process_keyword_line(line),
        Some(Token::Identifier(_)) => {
            // TODO: Save the value in the map
            // map[variable] = evaluate_expr(&tokens)
            if let Some(val) = evaluate_expr(&tokens) {
                println!("{}", val)
            }
        }
        Some(Token::Value(_)) => {
            if let Some(val) = evaluate_expr(&tokens) {
                println!("{}", val)
            }
        }
        Some(Token::Operator(op)) => match op {
            Operator::Plus => {}
            Operator::Minus => {}
            Operator::DoubleQuote => {
                if !matches!(tokens.last(), Some(Token::Operator(Operator::DoubleQuote))) {
                    print_error("Invalid expr,  starts with \" so should end with \".");
                }

                print_tokens(&tokens[1..tokens.len() - 1]);
            }
            _ => print_error(&format!(
                "Invalid start of the expr, syntax error use operator: {}",
                op
            )),
        },
        Some(Token::LParen) => {}
        Some(Token::RParen) => {}
        Some(Token::LBrace) => {}
        Some(Token::RBrace) => {}
        Some(Token::Semicolon) => {}
        Some(Token::Comma) => {}
        Some(Token::EOF) => {}
        Some(Token::Unknown) => {}
        None => {}
    }

    print_tokens(&tokens);
}

pub fn process_keyword_line(line: &str) {
    println!("{}", line);
}
