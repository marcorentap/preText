#[derive(Debug, Clone)]
pub enum ExpressionBase {
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    SubExpression(Box<Expression>),
    Array(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation(BinaryOp, Box<Expression>, Box<Expression>),
    PostfixOperation(PostfixOp, self::ExpressionBase),
    PrefixOperation(PrefixOp, Box<self::Expression>),
    FunctionCall(self::ExpressionBase, Vec<Expression>),
    VarIndexing(self::ExpressionBase, Box<Expression>),
    ExpressionBase(self::ExpressionBase),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
    AddAssign,
    SubAssign,
    MultAssign,
    DivAssign,
    Assign,
    Equals,
    NotEquals,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum PostfixOp {
    Incr,
    Decr,
    Call,
    Index,
}

#[derive(Debug, Clone)]
pub enum PrefixOp {
    Negative,
    Positive,
    Not,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(String, Expression),
    Include(Vec<ExpressionBase>, Option<ExpressionBase>),
    FileInclude(ExpressionBase, Option<ExpressionBase>),
    FilePartialInclude(ExpressionBase, Vec<ExpressionBase>, Option<ExpressionBase>),
    ExprStatement(Expression),
    VariableDefinition(ExpressionBase, Expression),
    SectionDefinition(ExpressionBase, Option<Vec<ExpressionBase>>, Vec<Statement>),
    SimpleSectionDefinition(ExpressionBase, Option<Vec<ExpressionBase>>, Expression),
    Conditional(Expression, Vec<Statement>, Option<Vec<Statement>>),
}
