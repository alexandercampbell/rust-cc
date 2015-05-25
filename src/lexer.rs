
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

