#[derive(Debug, Clone)]
pub enum ExpressionBase {
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    SubExpression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation(BinaryOp, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOp, String),
    FunctionCall(String, Vec<(String, Expression)>),
    ExpressionBase(self::ExpressionBase),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mult,
    Assign,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Increment,
    Decrement,
    Negate,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Section(String, Vec<String>, Vec<Statement>),
    Assignment(String, Expression),
    Include(String),
    ExprStatement(Expression),
    VariableDefinition(String, Expression),
}
