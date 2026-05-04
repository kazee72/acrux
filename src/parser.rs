use crate::{ast::{Program, Statement}, lexer::Lexer, token::{Token, TokenKind}};



struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
}



impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut lexer = lexer;
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser { lexer, current: current_token, peek: peek_token }
    }

    fn advance(&mut self) {
        self.current = std::mem::replace(&mut self.peek, self.lexer.next_token());
    }

    fn current_is(&self, expected: TokenKind) -> bool {
        expected == self.current.kind
    }

    fn peek_is(&self, expected: TokenKind) -> bool {
        expected == self.peek.kind
    }

    fn expect(&mut self, expected: TokenKind) {
        if self.current.kind == expected {
            self.advance();
        } else {
            panic!("Syntax Error: expected {:?}, got {:?} instead on line {}", expected, self.current.kind, self.current.line);
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        loop {
            if self.current.kind == TokenKind::EOF {
                break;
            } else {
                let stmt = self.parse_statement();
                statements.push(stmt);
            }
        }

        Program { statements }
    }

    fn parse_statement(&self) -> Statement {
        todo!();
    }

}