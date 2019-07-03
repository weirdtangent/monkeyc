#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfLine,

    // Literals
    Ident(String),
    Integer(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Bang,
    Slash,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}

#[test]
fn lookup_ident_test() {
    assert_eq!(lookup_ident("fn"), Token::Function);
}
