
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

/**
 * Declaration is a complete variable declaration. A Declaration may represent such complicated
 * statements such as:
 *
 * ```
 * int **a[12] = { 0 };
 * ```
 *
 */
#[derive(Clone,Debug,PartialEq)]
pub struct Declaration {
    pub _type:              Type,
    pub variable:           String,
    pub initial_value:      Expression,
}

/**
 * Type represents a construct such as `unsigned int` or `const FILE*`. Note that Type does not
 * include a variable name. See Declaration for that.
 */
#[derive(Clone,Debug,PartialEq)]
pub struct Type {
    pub base_name:      String,         // type name like `FILE` or `int`
    pub modifiers:      Vec<String>,    // modifiers like "unsigned", "long", "const", etc.
    pub length:         Option<usize>,  // for array declarations. `None` for simple variables.
    pub pointer_levels: usize,          // 0=value, 1=pointer, 2=pointer pointer, etc.
}

#[derive(Clone,Debug,PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Clone,Debug,PartialEq)]
pub struct Function {
    pub name:        String,
    pub return_type: Type,
    pub statements:  Vec<Statement>,
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

