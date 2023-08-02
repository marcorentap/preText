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
    PostfixOperation(PostfixOp, self::ExpressionBase),
    FunctionCall(self::ExpressionBase, Vec<Expression>),
    VarIndexing(self::ExpressionBase, Box<Expression>),
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
pub enum PostfixOp {
    Incr,
    Decr,
    Call,
    Index,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Section(String, Vec<String>, Vec<Statement>),
    Assignment(String, Expression),
    Include(Vec<ExpressionBase>, Option<ExpressionBase>),
    ExprStatement(Expression),
    VariableDefinition(ExpressionBase, Expression),
    SectionDefinition(ExpressionBase, Option<Vec<ExpressionBase>>, Vec<Statement>),
}
