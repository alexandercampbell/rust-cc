
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
// intentionally non-clonable; it would be confusing as hell to have multiple contexts floating
// around on the same vector of tokens
#[derive(Debug)]
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
        if self.pos >= self.tokens.len() {
            self.pos += 1; // so step_back does exactly what we expect every time.
            None
        } else {
            let next_token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(next_token)
        }
    }

    pub fn step_back(&mut self) {
        if self.pos > 0 { self.pos -= 1; }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.pos].clone())
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

