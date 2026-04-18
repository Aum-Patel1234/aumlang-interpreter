use crate::{
    ast::evaluate_expr,
    parser::{get_tokens, print_tokens},
    token::Token,
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
            println!("{}", evaluate_expr(&tokens));
        }
        Some(_) => {
            println!("{}", evaluate_expr(&tokens));
        }
        None => {}
    }

    print_tokens(&tokens);
}

pub fn process_keyword_line(line: &str) {
    println!("{}", line);
}
