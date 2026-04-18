use aumlang::{
    parser::get_tokens,
    token::{Keyword, Operator, Token, Value},
};

#[test]
fn test_print_keyword() {
    let tokens = get_tokens("print");

    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Print)));
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

    assert!(matches!(tokens[0], Token::Keyword(Keyword::Print)));
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
