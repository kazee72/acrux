use crate::{ast::{Expression, Program, Statement}, lexer::Lexer, token::{Token, TokenKind}};



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

    fn expect_identifier(&mut self) -> String {
        if let TokenKind::Identifier(name) = &self.current.kind {
            let name = name.to_string();
            self.advance();
            name
        } else {
            panic!("Syntax Error: expected Identifier, got {:?} instead on line {}", self.current.kind, self.current.line);
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

    fn parse_statement(&mut self) -> Statement {

        match self.current.kind {
            TokenKind::Affix => self.parse_variable_declaration(),
            TokenKind::Crux => self.parse_function_declaration(),
            TokenKind::Apex => self.parse_return(),
            TokenKind::Vox => self.parse_print(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            _ => self.parse_expression_statement()
        }
    }
    
    fn parse_variable_declaration(&mut self) -> Statement {

        self.advance();
        let var_name = self.expect_identifier();
        self.advance();
        self.expect(TokenKind::Assign);
        let value = self.parse_expression();
        self.expect(TokenKind::Semicolon);

        Statement::VariableDeclaration { name: var_name, value }
    }

    fn parse_function_declaration(&mut self) -> Statement {

        self.advance();
        let func_name = self.expect_identifier();
        self.expect(TokenKind::LParen);
        let mut params: Vec<String> = Vec::new();

        while !self.current_is(TokenKind::RParen) {
            let param_name = self.expect_identifier();
            params.push(param_name);
            if self.current_is(TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(TokenKind::RParen);
        self.expect(TokenKind::LCurly);

        let mut body: Vec<Statement> = Vec::new();

        while !self.current_is(TokenKind::RCurly) {
            if self.current_is(TokenKind::EOF) {
                panic!("Syntax Error: expected '}}', '{{' never closed")
            } else {
                let stmt = self.parse_statement();
                body.push(stmt);
            }
        }

        self.advance();

        Statement::FunctionDeclaration { name: func_name, parameters: params, body }
        
    }

    fn parse_return(&mut self) -> Statement {
        
        self.advance();
        let exp = self.parse_expression();
        self.expect(TokenKind::Semicolon);

        Statement::Return(exp)
    }

    fn parse_print(&mut self) -> Statement {
        
        self.advance();
        let exp = self.parse_expression();
        self.expect(TokenKind::Semicolon);

        Statement::Print(exp)
    }

    fn parse_if(&mut self) -> Statement {
        
        self.advance();
        self.expect(TokenKind::LParen);
        let condition = self.parse_expression();
        self.expect(TokenKind::RParen);
        self.expect(TokenKind::LCurly);

        let mut consequence_stmts: Vec<Statement> = Vec::new();

        while !self.current_is(TokenKind::RCurly) {
            if self.current_is(TokenKind::EOF) {
                panic!("Syntax Error: expected '}}', '{{' never closed")
            } else {
                let stmt = self.parse_statement();
                consequence_stmts.push(stmt);
            }
        }

        self.advance();

        let alternative = if self.current_is(TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::LCurly);
            let mut stmts: Vec<Statement> = Vec::new();
            while !self.current_is(TokenKind::RCurly)  {
                if self.current_is(TokenKind::EOF) {
                    panic!("Syntax Error: expected '}}', '{{' never closed")
                }
                stmts.push(self.parse_statement());
            }
            self.advance();
            Some(stmts)
        } else {
            None
        };

        Statement::If { condition, consequence: consequence_stmts, alternative }       
    }

    fn parse_while(&mut self) -> Statement {
        todo!()
    }

    fn parse_for(&mut self) -> Statement {
        todo!()
    }

    fn parse_expression_statement(&mut self) -> Statement {
        todo!()
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    

}