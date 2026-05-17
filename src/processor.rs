use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    eval::evaluate::eval,
    lexer::Lexer,
    object::obj::{Object, ObjectTrait},
    parser::parser_logic::Parser,
};

pub fn process_input(input: &str, env: Rc<RefCell<Environment>>) {
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
    // for stmt in &program.statements {
    //     println!("{:?}", stmt.string());
    // }
    // println!("{}", program);
    let evaluated = eval(&program, env);
    if let Some(e) = evaluated {
        match e {
            Object::Error(error_object) => {
                println!("\x1b[31mERROR: {}\x1b[0m", error_object.msg);
            }
            Object::Function(_) => {}
            _ => {
                println!("{}", e.inspect());
            }
        }
    }

    // print_tokens(&tokens);
}

pub fn process_keyword_line(line: &str) {
    println!("{}", line);
}
