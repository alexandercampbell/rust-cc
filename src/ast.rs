
#[derive(Clone,Debug,PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Modulo,
}

#[derive(Clone,Debug,PartialEq)]
pub enum UnaryOp {
    Reference,
    Dereference,
    Negate,
}

#[derive(Clone,Debug,PartialEq)]
pub enum Expression {
    Assignment(Box<Expression>, Box<Expression>),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    Variable(String),
    FunctionCall{name: String, args: Vec<Expression>},
    MemberAccess{struct_name: Box<Expression>, field_name: String},
    ArrayIndex{array: Box<Expression>, index: Box<Expression>},
}

#[derive(Clone,Debug,PartialEq)]
pub struct Declaration {
    pub _type:              String,
    pub variable:           String,
    pub length:             Option<usize>, // potentially an array declaration
    pub initial_value:      Expression,
}

#[derive(Clone,Debug,PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Clone,Debug,PartialEq)]
pub struct Function {
    pub name:       String,
    pub statements: Vec<Statement>,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Program {
    pub globals:    Vec<Declaration>,
    pub functions:  Vec<Function>,

    /*
     * TODO: Implement:
     *
     * pub structs:    Vec<Struct>,
     * pub typedefs:   Vec<Typedef>,
     */
}

