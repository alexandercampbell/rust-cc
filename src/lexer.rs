
/**
 * Each of the variants in this enum represents one kind of C token.
 */
pub enum Token {
    /// String is a literal pulled directly from source, however, it will probably have some
    /// special processing done for escape sequences.
    String(String),

    /// Number is a literal pulled directly from source.
    Number(Number),

    /// Identifier could be a function call or variable, or even type declaration.
    Identifier(String),

    /// Operators have to be handled with correct precedence, but that's a problem for the parser.
    /// The lexer is only concerned with defining which Operators exist.
    Operator(Operator),

    /*
     * The Tokens below this point should be self-explanatory :)
     */
    Comma,
    Semicolon,
    LParen, RParen,
    LBrace, RBrace,
}

/**
 * These have a direct correspondence to the C operators of the same name.
 *
 * TODO: Bitwise operators.
 */
pub enum Operator {
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    And,        // &&
    Or,         // ||
    Assignment, // =
}

/**
 * Number describes the possible **literals** that can occur as a result of lexing.
 *
 * Presently, we don't try to support literals like "10L" or "10.0f".
 * TODO: Implement those.
 */
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Operator {
    /**
     * Convert from the C literal of an operator to an Operator. If no such Operator exists, return
     * None.
     */
    pub fn from_str(s: &str) -> Option<Operator> {
        use lexer::Operator::*;

        Some(match s {
            "+" => Add,
            "-" => Subtract,
            "*" => Multiply,
            "/" => Divide,
            "&&" => And,
            "||" => Or,
            "=" => Assignment,
            _ => return None,
        })
    }
}

/**
 * Convert from a str to a vector of Tokens.
 *
 * For example, the string `", ( {"` would be transformed into the Vector of Tokens
 * `vec![Comma, LParen, LBrace]`.
 *
 * The result of this function is just a sequence of Token without hierarchy. These Tokens should
 * be parsed to build a walkable AST.
 */
pub fn lex(s: &str) -> Result<Vec<Token>, &'static str> {
    Err("not yet implemented")
}

