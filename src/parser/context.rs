
use lexer::Token;

/**
 * Context is a thin wrapper around Vec<Token> that has special support for some navigational
 * logic. Context could technically implement Iterator, but I've chosen not to do that as it would
 * complicate usage of the struct.
 *
 * The use case of Context is different from an Iterator anyway. With an Iterator you
 *
 * 1. Only move forward.
 * 2. Perform a similar operation on each Item.
 *
 * A Context is designed for careful pattern-matching and reversal to a set point when
 * pattern-matching fails. These "set points" are called Checkpoints. It's kinda dumb but at least
 * it's easy to understand and it works.
 */
pub struct Context {
    tokens:             Vec<Token>,
    pos:                usize,
}

impl Context {
    pub fn new(tokens: Vec<Token>) -> Self {
        Context{
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.pos += 1;
        if self.pos >= self.tokens.len() {
            // prevent multiple calls to next() at the end of a token stream from advancing us
            // beyond self.tokens.len();
            self.pos = self.tokens.len();
            None
        } else {
            Some(self.tokens[self.pos].clone())
        }
    }

    pub fn peek(&self) -> Option<Token> {
        let next_pos = self.pos + 1;
        if next_pos >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[next_pos].clone())
        }
    }

    pub fn make_checkpoint(&mut self) -> Checkpoint {
        Checkpoint{saved_pos: self.pos}
    }
}

/**
 * Checkpoint records a position in a Context and is capable of restoring to that position.
 */
pub struct Checkpoint {
    saved_pos: usize,
}

impl Checkpoint {
    pub fn restore(self, context: &mut Context) {
        context.pos = self.saved_pos;
    }
}

