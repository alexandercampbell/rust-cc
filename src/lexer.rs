
/**
 * Each of the variants in this enum represents one kind of C token.
 */
pub enum Token {
    Identifier(String),
    Operator(Operator),
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

