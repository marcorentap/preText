#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    BinaryOperation(BinaryOp, Box<Expression>, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Section(String, Vec<String>, Vec<Statement>),
    Assignment(String, Expression),
    Include(String),
    ExprStatement(Expression),
}
