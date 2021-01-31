use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Less,
    Greater,
    Plus,
    Minus,
    Period,
    Comma,
    LBracket,
    RBracket,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidChar(usize, char),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::InvalidChar(pos, c) => write!(f, "{}: invalid char '{}'", pos, c)
        }
    }
}

impl std::error::Error for LexError {}

pub fn lex(code: &str) -> Result<Vec<Token>, LexError> {
    let mut pos = 0;
    let input = code.as_bytes();
    let mut tokens = Vec::new();

    macro_rules! lex_token {
        ($kind:expr) => {{
            tokens.push($kind);
            pos += 1;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'+' => lex_token!(Token::Plus),
            b'-' => lex_token!(Token::Minus),
            b'>' => lex_token!(Token::Less),
            b'<' => lex_token!(Token::Greater),
            b'.' => lex_token!(Token::Period),
            b',' => lex_token!(Token::Comma),
            b'[' => lex_token!(Token::LBracket),
            b']' => lex_token!(Token::RBracket),
            b' ' | b'\n' | b'\t' => pos += 1,
            c => return Err(LexError::InvalidChar(pos, c as char)),
        }
    }

    Ok(tokens)
}
