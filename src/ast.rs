#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    BinaryOperation(BinaryOp, Box<Expression>, Box<Expression>),
    FunctionCall(String, Vec<(String, Expression)>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Assign,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Section(String, Vec<String>, Vec<Statement>),
    Assignment(String, Expression),
    Include(String),
    ExprStatement(Expression),
}
