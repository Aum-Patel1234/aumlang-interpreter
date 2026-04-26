use crate::{lexer::Lexer, parser::parser_logic::Parser};

pub fn process_input(input: &str) {
    let lexer = Lexer::new_lexer(input);
    // let mut tokens: Vec<Token> = Vec::new();
    // loop {
    //     let token = lexer.next_token();
    //
    //     if token == Token::EOF {
    //         tokens.push(token);
    //         break;
    //     }
    //
    //     tokens.push(token);
    // }
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    parser.check_parse_errors();
    println!("{:#?}", program);

    // print_tokens(&tokens);
}

pub fn process_keyword_line(line: &str) {
    println!("{}", line);
}
