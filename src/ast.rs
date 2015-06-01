/*!
 * AST
 * ===
 *
 * This module describes the core datastructures of our compiler. The structures in this file can
 * be used to represent an entire C program.
 *
 * TODO:
 *
 *  - Implement parser that can convert a stream of tokens into an `ast::Program`.
 *  - Implement interpreter that can run any Program. There are some quirks such as the C standard
 *    library which may be difficult to interop correctly.
 *  - Implement compiler that can compile a Program into either assembly or machine code. This will
 *    depend on platform and will likely be the hardest step.
 *  - Implement "reverser" that can deterministically output an `ast::Program` as valid C source
 *    code.
 *
 */

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
    Declaration(Declaration),
}

/**
 * Declaration is a complete variable declaration. A Declaration may represent such complicated
 * statements such as:
 *
 * ```
 * int **a[12];
 * ```
 *
 */
#[derive(Clone,Debug,PartialEq)]
pub struct Declaration {
    pub _type:              Type,
    pub variable:           String,
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
    Return(Expression),
    Continue,
    Break,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Function {
    pub name:        String,
    pub arguments:   Vec<Declaration>,
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

