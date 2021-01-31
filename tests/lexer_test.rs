use brainfuck::lexer::{lex, Token, LexError};

#[test]
fn lex_simple_code() {
    let code = ">+-<";
    let tokens = lex(code).unwrap();

    assert_eq!(tokens, vec![
        Token::Less,
        Token::Plus,
        Token::Minus,
        Token::Greater,
    ])
}

#[test]
fn lex_complex_code() {
    let code = ">++++++[<++++++++>-].";
    let tokens = lex(code).unwrap();

    assert_eq!(tokens, vec![
        Token::Less,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::LBracket,
        Token::Greater,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Plus,
        Token::Less,
        Token::Minus,
        Token::RBracket,
        Token::Period,
    ])
}

#[test]
fn lex_invalid_char() {
    let code = "+++!";
    let result = lex(code);
    
    assert!(result.is_err());
    assert_eq!(result, Err(LexError::InvalidChar(3, '!')))
}
