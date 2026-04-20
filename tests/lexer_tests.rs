use aumlang::{
    lexer::{get_tokens, Lexer},
    token::{Keyword, Operator, Token, Value},
};

#[test]
fn test_print_keyword() {
    let tokens = get_tokens("print");

    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::PRINT)));
}

#[test]
fn test_operators() {
    let tokens = get_tokens("+ - * / =");

    assert_eq!(tokens.len(), 5);

    assert!(matches!(tokens[0], Token::Operator(Operator::Plus)));
    assert!(matches!(tokens[1], Token::Operator(Operator::Minus)));
    assert!(matches!(tokens[2], Token::Operator(Operator::Star)));
    assert!(matches!(tokens[3], Token::Operator(Operator::Slash)));
    assert!(matches!(tokens[4], Token::Operator(Operator::Equal)));
}

#[test]
fn test_numbers() {
    let tokens = get_tokens("10 3.14");

    assert_eq!(tokens.len(), 2);

    match tokens[0] {
        Token::Value(Value::Double(v)) => assert_eq!(v, 10.0),
        _ => panic!("Expected number"),
    }

    match tokens[1] {
        Token::Value(Value::Double(v)) => assert_eq!(v, 3.14),
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_identifier() {
    let tokens = get_tokens("hello_world");

    assert_eq!(tokens.len(), 1);

    match &tokens[0] {
        Token::Identifier(name) => assert_eq!(name, "hello_world"),
        _ => panic!("Expected identifier"),
    }
}

#[test]
fn test_expression() {
    let tokens = get_tokens("print(x + 5)");

    assert_eq!(tokens.len(), 6);

    assert!(matches!(tokens[0], Token::Keyword(Keyword::PRINT)));
    assert!(matches!(tokens[1], Token::LParen));
    assert!(matches!(tokens[2], Token::Identifier(_)));
    assert!(matches!(tokens[3], Token::Operator(Operator::Plus)));
    assert!(matches!(tokens[4], Token::Value(_)));
    assert!(matches!(tokens[5], Token::RParen));
}

#[test]
fn test_invalid_char() {
    let tokens = get_tokens("@");

    assert_eq!(tokens.len(), 0);
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
"#;

    let tests = vec![
        Token::Keyword(Keyword::LET),
        Token::Identifier("five".to_string()),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Double(5.0)),
        Token::Semicolon,
        Token::Keyword(Keyword::LET),
        Token::Identifier("ten".to_string()),
        Token::Operator(Operator::Equal),
        Token::Value(Value::Double(10.0)),
        Token::Semicolon,
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
