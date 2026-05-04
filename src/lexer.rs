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

    fn read_number(&mut self) -> Token {
        let mut is_float = false;
        let mut collected_number_string = String::new();

        loop {
            match self.peek(self.pos) {
                Some(ch) => {
                    if ch.is_ascii_digit() {
                        collected_number_string.push(ch);
                    } else if ch == '.' && !is_float {
                        is_float = true;
                        collected_number_string.push(ch);
                    } else {
                        break;
                    }
                },
                _ => break
            }
            self.pos += 1;
        }

        if is_float {
            let collected_float = collected_number_string.parse::<f64>().unwrap();
            Token { kind: TokenKind::Float(collected_float), line: self.line }
        } else {
            let collected_int = collected_number_string.parse::<i64>().unwrap();
            Token { kind: TokenKind::Integer(collected_int), line: self.line }
        }


    }

    fn read_string(&mut self) -> Token {
        let mut collected_string = String::new();

        self.pos += 1;

        loop {
            match self.peek(self.pos) {
                Some('"') => {
                    self.pos += 1;
                    break;
                },
                Some(ch) => {
                    if ch == '\n' { self.line += 1; }
                    collected_string.push(ch);
                    self.pos += 1;
                },
                None => break
            }
        }

        Token { kind: TokenKind::StringLit(collected_string), line: self.line }
    }
    
    fn lookup_keyword(keyword: &str) -> TokenKind {
        match keyword {
            "affix" => TokenKind::Affix,
            "crux" => TokenKind::Crux,
            "vox" => TokenKind::Vox,
            "apex" => TokenKind::Apex,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "true" => TokenKind::True,
            "false" => TokenKind::False,

            _ => TokenKind::Identifier(keyword.to_string())
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut collected_string = String::new();

        loop {
            match self.peek(self.pos) {
                Some(ch) => {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        collected_string.push(ch);
                    } else {
                        break;
                    }
                }

                _ => break
            }

            self.pos += 1;
        }

        let token_kind = Lexer::lookup_keyword(collected_string.as_str());

        Token { kind: token_kind, line: self.line }
    }

    
}

