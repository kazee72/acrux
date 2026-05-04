


struct Program {
    statements: Vec<Statement>
}

enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Expression),
    Print(Expression),
    If {
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        initializer: Box<Statement>,
        condition: Expression,
        increment: Expression,
        body: Vec<Statement>,
    },
    ExpressionStatement(Expression)
}

enum Expression {
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolenLiteral(bool),
    Null,
    Identifier(String),
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryExpression {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Assign {
        name: String,
        value: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    }
}

enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or
}

enum UnaryOperator {
    Minus,
    Not
}