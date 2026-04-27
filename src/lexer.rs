use crate::token::Token;
use crate::token::TokenKind;


pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize
}

impl Lexer {
    pub fn new(source_string: String) -> Self {
        Lexer { 
            source: source_string.chars().collect(),
            pos: 0,
            line: 1
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if let Some(ch) = self.peek(self.pos) {
            match ch {
                '+' => self.make_token(TokenKind::Plus, 1),
                '-' => self.make_token(TokenKind::Minus, 1),
                '*' => self.make_token(TokenKind::Mult, 1),
                '/' => self.make_token(TokenKind::Div, 1),
                '(' => self.make_token(TokenKind::LParen, 1),
                ')' => self.make_token(TokenKind::RParen, 1),
                '{' => self.make_token(TokenKind::LCurly, 1),
                '}' => self.make_token(TokenKind::RCurly, 1),
                ',' => self.make_token(TokenKind::Comma, 1),
                ';' => self.make_token(TokenKind::Semicolon, 1),
                '!' => {
                    match self.peek(self.pos + 1) {
                        Some('=') => self.make_token(TokenKind::NotEq, 2),
                        _ => self.make_token(TokenKind::Not, 1),
                    }
                },
                '<' =>  {
                    match self.peek(self.pos + 1) {
                        Some('=') => self.make_token(TokenKind::LtEq, 2),
                        _ => self.make_token(TokenKind::Lt, 1),
                    }
                },
                '>' => {
                    match self.peek(self.pos + 1) {
                        Some('=') => self.make_token(TokenKind::GtEq, 2),
                        _ => self.make_token(TokenKind::Gt, 1),
                    }
                },

                '=' => {
                    match self.peek(self.pos + 1) {
                        Some('=') => self.make_token(TokenKind::Eq, 2),
                        _ => self.make_token(TokenKind::Assign, 1),
                    }
                },
                '&' => {
                    match self.peek(self.pos + 1) {
                        Some('&') => self.make_token(TokenKind::And, 2),
                        _ => self.make_token(TokenKind::Unknown('&'), 1),
                    }
                },
                '|' => {
                    match self.peek(self.pos + 1) {
                        Some('|') => self.make_token(TokenKind::Or, 2),
                        _ => self.make_token(TokenKind::Unknown('|'), 1),
                    }
                },
                '"' => self.read_string(),
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),

                _ => {
                    let ch = self.peek(self.pos).unwrap();
                    self.make_token(TokenKind::Unknown(ch), 1)
                }
            }
        } else {
            Token { kind: TokenKind::EOF, line: self.line }
        }
        
    }

    fn make_token(&mut self, kind: TokenKind, increase: usize) -> Token {
        self.pos += increase;
        Token { kind, line: self.line }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if let Some(ch) = self.peek(self.pos) {
                match ch {
                    ' ' | '\t' | '\r' => self.pos += 1,
                    '\n' => {self.line += 1; self.pos += 1;},
                    '/' =>  if let Some(ch_next) = self.peek(self.pos + 1) {
                                if ch_next == '/' {
                                    while let Some(c) = self.peek(self.pos) {
                                        if c == '\n' {break;}
                                        self.pos += 1;
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            },
                    _   => break
                }
            } else {
                break;
            }
        }
    }

    fn peek(&self, pos: usize) -> Option<char> {
        self.source.get(pos).copied()
    }

    fn read_number(&self) -> Token {
        todo!()
    }

    fn read_string(&self) -> Token {
        todo!()
    }

    fn read_identifier(&self) -> Token {
        todo!()
    }
}

