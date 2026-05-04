#[derive(PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}


#[derive(PartialEq, Debug)]
pub enum TokenKind {
    // Keywords
    Affix,
    Crux,
    Vox,
    Apex,
    If,
    Else,
    While,
    For,
    True,
    False,
    // Operators
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    And,
    Or,
    Not,
    Gt,
    Lt,
    Eq,
    NotEq,
    GtEq,
    LtEq,
    // Delimiters
    Semicolon,
    Comma,
    LParen,
    RParen,
    LCurly,
    RCurly,
    // Literals
    Integer(i64),
    Float(f64),
    StringLit(String),
    Identifier(String),
    // Other
    EOF,
    Unknown(char)
}