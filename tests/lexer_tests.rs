use aumlang::{
    lexer::Lexer,
    token::{Keyword, Operator, Token, Value},
};

#[test]
fn test_print_keyword() {
    let input = "print";
    let tests = [Token::Keyword(Keyword::PRINT), Token::EOF];
    let mut lexer = Lexer::new_lexer(input);
    for (i, expected) in tests.iter().enumerate() {
        let tok = lexer.next_token();

        assert_eq!(
            tok, *expected,
            "Test {} failed: expected {:?}, got {:?}",
            i, expected, tok
        );
    }
}
#[test]
fn test_operators() {
    let input = "+ - * / =";
    let tests = [
        Token::Operator(Operator::Plus),
        Token::Operator(Operator::Minus),
        Token::Operator(Operator::Star),
        Token::Operator(Operator::Slash),
        Token::Operator(Operator::Equal),
        Token::EOF,
    ];
    let mut lexer = Lexer::new_lexer(input);
    for (i, expected) in tests.iter().enumerate() {
        let tok = lexer.next_token();

        assert_eq!(
            tok, *expected,
            "Test {} failed: expected {:?}, got {:?}",
            i, expected, tok
        );
    }
}

// TODO: add support for 3.14 curent takes 3.0
#[test]
fn test_numbers() {
    let input = "#10 3.14";
    let mut lexer = Lexer::new_lexer(input);
    // match lexer.next_token() {
    //     Token::Value(Value::Double(v)) => assert_eq!(v, 10.0),
    //     tok => panic!("Expected 10, got {:?}", tok),
    // }
    // match lexer.next_token() {
    //     Token::Value(Value::Double(v)) => assert_eq!(v, 3.14),
    //     tok => panic!("Expected 3.14, got {:?}", tok),
    // }
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_identifier() {
    let input = "hello_world";
    let mut lexer = Lexer::new_lexer(input);
    match lexer.next_token() {
        Token::Identifier(name) => assert_eq!(name, "hello_world"),
        tok => panic!("Expected identifier, got {:?}", tok),
    }
    assert_eq!(lexer.next_token(), Token::EOF);
}

#[test]
fn test_expression() {
    let input = "print(x + 5)";
    let tests = vec![
        Token::Keyword(Keyword::PRINT),
        Token::LParen,
        Token::Identifier("x".to_string()),
        Token::Operator(Operator::Plus),
        Token::Value(Value::Double(5.0)),
        Token::RParen,
        Token::EOF,
    ];

    let mut lexer = Lexer::new_lexer(input);
    for (i, expected) in tests.iter().enumerate() {
        let tok = lexer.next_token();

        assert_eq!(
            tok, *expected,
            "Test {} failed: expected {:?}, got {:?}",
            i, expected, tok
        );
    }
}

#[test]
fn test_invalid_char() {
    let input = "@";
    let mut lexer = Lexer::new_lexer(input);
    let mut tok = lexer.next_token();
    assert_eq!(tok, Token::Unknown);
    tok = lexer.next_token();
    assert_eq!(tok, Token::EOF);
}

#[test]
fn test_next_token() {
    let input = "=+(){};";
    let tests = vec![
        Token::Operator(Operator::Equal),
        Token::Operator(Operator::Plus),
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        // Token::Comma,
        Token::Semicolon,
        Token::EOF, // EOF
    ];

    let mut lexer = Lexer::new_lexer(input);
    for (i, expected) in tests.iter().enumerate() {
        let token = lexer.next_token();

        assert_eq!(
            token, *expected,
            "Test {} failed: expected {:?}, got {:?}",
            i, expected, token
        )
    }
}

#[test]
fn test_next_token_words() {
    let input = r#"
let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
"#;

    let tests = vec![
        // let five = 5;
        Token::Keyword(Keyword::LET),
        Token::Identifier("five".to_string()),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Double(5.0)),
        Token::Semicolon,
        // let ten = 10;
        Token::Keyword(Keyword::LET),
        Token::Identifier("ten".to_string()),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Double(10.0)),
        Token::Semicolon,
        // let add = fn(x, y) { x + y; };
        Token::Keyword(Keyword::LET),
        Token::Identifier("add".to_string()),
        Token::Operator(Operator::Equal),
        Token::Keyword(Keyword::FUNCTION),
        Token::LParen,
        Token::Identifier("x".to_string()),
        Token::Comma,
        Token::Identifier("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Identifier("x".to_string()),
        Token::Operator(Operator::Plus),
        Token::Identifier("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        // let result = add(five, ten);
        Token::Keyword(Keyword::LET),
        Token::Identifier("result".to_string()),
        Token::Operator(Operator::Equal),
        Token::Identifier("add".to_string()),
        Token::LParen,
        Token::Identifier("five".to_string()),
        Token::Comma,
        Token::Identifier("ten".to_string()),
        Token::RParen,
        Token::Semicolon,
        // !-/*5;
        Token::Operator(Operator::Exclamation),
        Token::Operator(Operator::Minus),
        Token::Operator(Operator::Slash),
        Token::Operator(Operator::Star),
        Token::Value(Value::Double(5.0)),
        Token::Semicolon,
        // 5 < 10 > 5;
        Token::Value(Value::Double(5.0)),
        Token::Operator(Operator::LT),
        Token::Value(Value::Double(10.0)),
        Token::Operator(Operator::GT),
        Token::Value(Value::Double(5.0)),
        Token::Semicolon,
        // if (5 < 10) { return true; } else { return false; }
        Token::Keyword(Keyword::IF),
        Token::LParen,
        Token::Value(Value::Double(5.0)),
        Token::Operator(Operator::LT),
        Token::Value(Value::Double(10.0)),
        Token::RParen,
        Token::LBrace,
        Token::Keyword(Keyword::RETURN),
        Token::Keyword(Keyword::TRUE),
        Token::Semicolon,
        Token::RBrace,
        Token::Keyword(Keyword::ELSE),
        Token::LBrace,
        Token::Keyword(Keyword::RETURN),
        Token::Keyword(Keyword::FALSE),
        Token::Semicolon,
        Token::RBrace,
        // 10 == 10;
        Token::Value(Value::Double(10.0)),
        Token::Operator(Operator::EQ),
        Token::Value(Value::Double(10.0)),
        Token::Semicolon,
        // 10 != 9;
        Token::Value(Value::Double(10.0)),
        Token::Operator(Operator::NEQ),
        Token::Value(Value::Double(9.0)),
        Token::Semicolon,
        Token::EOF,
    ];

    let mut lexer = Lexer::new_lexer(input);

    for (i, expected_token) in tests.iter().enumerate() {
        let tok = lexer.next_token();

        assert_eq!(
            format!("{:?}", tok),
            format!("{:?}", expected_token),
            "tests[{}] - token mismatch. expected={:?}, got={:?}",
            i,
            expected_token,
            tok
        );
    }
}
