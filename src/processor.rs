use crate::{
    lexer::{Lexer, print_tokens},
    token::Token,
};

pub fn process_input(input: &str) {
    let mut lexer = Lexer::new_lexer(input);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.next_token();

        if token == Token::EOF {
            tokens.push(token);
            break;
        }

        tokens.push(token);
    }
    print_tokens(&tokens);
}

pub fn process_keyword_line(line: &str) {
    println!("{}", line);
}
