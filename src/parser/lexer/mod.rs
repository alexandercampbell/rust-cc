
use ast;
use util;

mod lex;

pub type Context = util::StepbackIterator<char>;

/**
 * Each of the variants in this enum represents one kind of C token.
 *
 * This enum contains no positional data.
 */
#[derive(Clone,Debug,PartialEq)]
pub enum Token {
    /// String is a literal pulled directly from source, however, it will probably have some
    /// special processing done for escape sequences. Example: `"hello"`
    String(String),

    /// Character is a single-byte literal from the source code. Examples: `'c'` or `'\n'`
    Character(char),

    /// Number is also a literal pulled directly from source. Examples: `98` or `3.14`
    Number(ast::Number),

    /// Identifier could be a function call or variable, or even type declaration. Example: `main`
    /// or `float`.
    Identifier(String),

    /// Operators have to be handled with correct precedence, but that's a problem for the parser.
    /// The lexer is only concerned with defining which Operators exist. Operators are either one
    /// or two characters.
    Operator(Operator),

    /*
     * The Tokens below this point should be self-explanatory :)
     */
    Comma,
    Period,
    Semicolon,
    LParen, RParen,
    LBrace, RBrace,
    LSquareBracket, RSquareBracket,
}

/**
 * These have a direct correspondence to the C operators of the same name.
 *
 * TODO: Bitwise and comparison operators.
 */
#[derive(Clone,Debug,PartialEq)]
pub enum Operator {
    /// Asterisk can be either multiplication or dereference, depending on parse context.
    Asterisk,  // *
    Add,       // +
    Subtract,  // -
    Divide,    // /
    Modulo,    // %
    And,       // &&
    Or,        // ||
    Assign,    // =
    Reference, // &
}

impl Operator {
    /**
     * Convert from the C literal of an operator to an Operator. If no such Operator exists, return
     * None. For example, this function would convert from `"*"` to `Operator::Asterisk`.
     */
    #[allow(dead_code)] // useful for documentation if nothing else
    pub fn from_str(s: &str) -> Option<Operator> {
        use self::Operator::*;

        Some(match s {
            "+" => Add,
            "-" => Subtract,
            "*" => Asterisk,
            "/" => Divide,
            "&&" => And,
            "||" => Or,
            "=" => Assign,
            _ => return None,
        })
    }
}

/**
 * Convert from a str to a vector of Tokens. Handle comments correctly as part of lexing.
 *
 * For example, the string `", ( {"` would be transformed into the Vector of Tokens
 * `vec![Comma, LParen, LBrace]`.
 *
 * The result of this function is just a sequence of Token without hierarchy. These Tokens should
 * be parsed to build a walkable AST.
 */
pub fn lex_str(s: &str) -> Result<Vec<Token>, String> {
    let chars:Vec<char> = s.chars().collect();
    let mut context = Context::new(chars);
    let mut tokens = vec![];

    // use an anonymous scope here so `push_tok` is dropped before we `Ok(tokens)`. Why? Because
    // for some reason, the drop keyword wasn't actually dropping `push_tok`.
    {
        let mut push_tok = |tok| {
            // debug printing can be added here to easily record
            //
            // 1. what new tokens are pushed.
            // 2. when they were pushed, relative to the other tokens.
            //
            tokens.push(tok);
        };

        // iterate through chars and process tokens as we go
        loop {
            let ch = match context.next() {
                Some(ch) => ch,
                None => break,
            };

            match ch {
                '0'...'9'|'.' => {
                    context.step_back();
                    let number = try!(lex::number(&mut context));
                    push_tok(number);
                },
                'a'...'z'|'A'...'Z'|'_' => {
                    context.step_back();
                    push_tok(Token::Identifier(lex::identifier(&mut context)));
                }
                '"' => push_tok(Token::String(try!(lex::string(&mut context)))),
                '\'' => (), // TODO: lex character

                // single-character tokens
                '{' => push_tok(Token::LBrace),
                '}' => push_tok(Token::RBrace),
                '[' => push_tok(Token::LSquareBracket),
                ']' => push_tok(Token::RSquareBracket),
                '(' => push_tok(Token::LParen),
                ')' => push_tok(Token::RParen),
                ',' => push_tok(Token::Comma),
                ';' => push_tok(Token::Semicolon),
                ' '|'\n'|'\t' => (), // ignore whitespace

                // TODO: more sophisticated operator lexing. Most of these can actually be
                // two-character operators.
                '+' => push_tok(Token::Operator(Operator::Add)),
                '-' => push_tok(Token::Operator(Operator::Subtract)),
                '*' => push_tok(Token::Operator(Operator::Asterisk)),
                '=' => push_tok(Token::Operator(Operator::Assign)),
                '%' => push_tok(Token::Operator(Operator::Modulo)),

                // comments are handled in this block
                '/' => {
                    match context.peek() {
                        Some('*') => {
                            // TODO: handle comment until a `*/` symbol
                        },

                        Some('/') => loop {
                            // comment till the end of the line
                            match context.next() {
                                // Backslash escapes newlines, even in comments. We know we can
                                // safely skip the next character no matter what it is.
                                Some('\\') => { context.next(); },
                                Some('\n') | None => break,
                                _ => (),
                            }
                        },

                        _ => push_tok(Token::Operator(Operator::Divide)),
                    }
                },

                _ => return Err(format!("unexpected character '{}'", ch)),
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;
    use ast;

    #[test]
    fn unexpected_character() {
        assert!(lex_str("$").is_err());
        assert!(lex_str("@").is_err());
    }

    #[test]
    fn numbers() {
        assert_eq!(lex_str("123").unwrap(), vec![Token::Number(ast::Number::Int(123))]);
        assert_eq!(lex_str("12.3").unwrap(), vec![Token::Number(ast::Number::Float(12.3))]);
        assert_eq!(lex_str("012").unwrap(), vec![Token::Number(ast::Number::Int(12))]);
        assert_eq!(lex_str("0120}").unwrap(), vec![Token::Number(ast::Number::Int(120)), Token::RBrace]);
    }

    #[test]
    fn identifiers() {
        assert_eq!(lex_str("int ident1, _ident2;").unwrap(),
            vec![
                Token::Identifier("int".to_string()),
                Token::Identifier("ident1".to_string()),
                Token::Comma,
                Token::Identifier("_ident2".to_string()),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn strings() {
        assert_eq!(lex_str(r##""\n\\\"""##).unwrap(), vec![Token::String("\n\\\"".to_string())]);
        assert!(lex_str("\"hello ").is_err());
        assert!(lex_str("\"hello \\").is_err());
    }

    #[test]
    fn one_line_comments() {
        assert_eq!(lex_str("").unwrap(), vec![]);
        assert_eq!(lex_str("//").unwrap(), vec![]);
        assert_eq!(lex_str("// hello ").unwrap(), vec![]);
        assert_eq!(lex_str("// hello \n\n").unwrap(), vec![]);
        assert_eq!(lex_str(", // hello").unwrap(), vec![Token::Comma]);
        assert_eq!(lex_str(", // hello \\\n goodbye").unwrap(), vec![Token::Comma]); // escaped newline
        assert_eq!(lex_str(", // hello \n ;").unwrap(), vec![Token::Comma, Token::Semicolon]);
    }

    #[test]
    fn simple_program() {
        let simple_program =
            r##"
                int main(int argc, char *argv[]) {
                    printf("Hello world\n");
                    return 0;
                }
            "##;

        let tokens = lex_str(simple_program).unwrap();
        assert_eq!(tokens,
            vec![
                Token::Identifier("int".to_string()),
                Token::Identifier("main".to_string()),
                Token::LParen,
                    Token::Identifier("int".to_string()),
                    Token::Identifier("argc".to_string()),
                    Token::Comma,
                    Token::Identifier("char".to_string()),
                    Token::Operator(Operator::Asterisk),
                    Token::Identifier("argv".to_string()),
                    Token::LSquareBracket,
                    Token::RSquareBracket,
                Token::RParen,
                Token::LBrace,
                    Token::Identifier("printf".to_string()),
                    Token::LParen,
                        Token::String("Hello world\n".to_string()),
                    Token::RParen,
                    Token::Semicolon,
                    Token::Identifier("return".to_string()),
                    Token::Number(ast::Number::Int(0)),
                    Token::Semicolon,
                Token::RBrace,
            ]
        );
    }
}

