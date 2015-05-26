
/**
 * Each of the variants in this enum represents one kind of C token.
 */
#[derive(Clone,Debug,PartialEq)]
pub enum Token {
    /// String is a literal pulled directly from source, however, it will probably have some
    /// special processing done for escape sequences. Example: `"hello"`
    String(String),

    /// Character is a single-byte literal from the source code. Examples: `'c'` or `'\n'`
    Character(char),

    /// Number is also a literal pulled directly from source. Examples: `98` or `3.14`
    Number(Number),

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
    Add,      // +
    Subtract, // -
    /// Asterisk can be either multiplication or dereference, depending on parse context.
    Asterisk, // *
    Divide,   // /
    And,      // &&
    Or,       // ||
    Assign,   // =
}

impl Operator {
    /**
     * Convert from the C literal of an operator to an Operator. If no such Operator exists, return
     * None. For example, this function would convert from `"*"` to `Operator::Asterisk`.
     */
    #[allow(dead_code)] // useful for documentation if nothing else
    pub fn from_str(s: &str) -> Option<Operator> {
        use lexer::Operator::*;

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
 * Number describes the possible **literals** that can occur as a result of lexing.
 *
 * Presently, we don't try to support literals like "10L" or "10.0f".
 * TODO: Implement those.
 */
// NOTE: changes to this enum require changes to `lex_number()`
#[derive(Clone,Debug,PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

/**
 * Retrieve the next `char` after `pos` if possible.
 */
fn peek_at_next_char(chars: &Vec<char>, pos: usize) -> Option<char> {
    let next_pos = pos + 1;
    if next_pos >= chars.len() {
        None
    } else {
        Some(chars[next_pos])
    }
}

/**
 * Extract as much of a number as possible from `chars`, starting at index `pos`. The `pos`
 * parameter will be mutated to point to the first token of the next character after the number.
 *
 * For example, if you have a string like this: " 12.3abc" and you start at `pos=2`, this function
 * will return `2.3` as a token and `pos` will be mutated to point to `a`.
 */
fn lex_number(chars: &Vec<char>, pos: &mut usize) -> Result<Number, String> {
    let first_ch = chars[*pos];
    let mut seen_decimal = first_ch == '.';
    let mut literal = String::new();
    literal.push(first_ch);

    loop {
        *pos += 1;
        if *pos >= chars.len() { break; }

        let ch = chars[*pos];
        match ch {
            '0'...'9' => (),
            '.' => {
                // If `seen_decimal` is already true, that means we've already seen a '.' character
                // in this literal.
                if seen_decimal {
                    return Err(format!("two decimals in numeric literal '{}.'", literal));
                };
                seen_decimal = true;
            }

            // TODO: handle trailing type specifiers like 10f and 50L (which mean float and long
            // respectively).
            _ => {
                // `chars[pos]` is part of another token. Back up `pos` so we don't refer to that
                // part of the string.
                *pos -= 1;
                break;
            },
        }
        literal.push(ch);
    };

    // We now have the literal contained in `literal`. Parse it into an instance of the Number
    // enum. Our variable `seen_decimal` tells us whether the number should be parsed as an int64
    // or a float64.
    if seen_decimal {
        let f = match literal.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(format!("bad floating point literal '{}'", literal)),
        };
        Ok(Number::Float(f))
    } else {
        let i = match literal.parse::<i64>() {
            Ok(i) => i,
            Err(_) => return Err(format!("bad integer literal '{}'", literal)),
        };
        Ok(Number::Int(i))
    }
}

/**
 * Lex as much of an identifer as possible from `chars`. Identifiers match the following regex:
 *
 * ```
 * [A-Za-z_][A-Za-z0-9_]*
 * ```
 *
 * This function assumes that the first character class has already been matched.
 */
fn lex_identifier(chars: &Vec<char>, pos: &mut usize) -> String {
    let mut literal = String::new();
    literal.push(chars[*pos]);

    loop {
        *pos += 1;
        if *pos >= chars.len() { break; }

        let ch = chars[*pos];
        match ch {
            'A'...'Z'|'a'...'z'|'0'...'9'|'_' => literal.push(ch),
            _ => {
                // `chars[pos]` is part of another token. Back up `pos` so we don't refer to that
                // part of the string.
                *pos -= 1;
                break;
            }
        };
    }

    return literal;
}

/**
 * Lex a string from `chars` starting at `pos`.
 *
 * NOTE: This function assumes that the first quote has been seen already.
 */
fn lex_string(chars: &Vec<char>, pos: &mut usize) -> Result<String, String> {
    let mut literal = String::new();

    loop {
        *pos += 1;
        if *pos >= chars.len() {
            return Err(format!("unterminated string literal {:?}", literal));
        }

        let ch = chars[*pos];
        match ch {
            '"' => return Ok(literal),
            '\\' => {
                // the next character must be an escape sequence
                match peek_at_next_char(chars, *pos) {
                    Some('"') => literal.push('"'),
                    Some('n') => literal.push('\n'),
                    Some('r') => literal.push('\r'),
                    Some('\\') => literal.push('\\'),
                    None => return Err(format!("EOF while scanning string literal {:?}", literal)),
                    Some(c) => return Err(format!("Unrecognized escape sequence \\{}", c)),
                };
                *pos += 1;
            },
            _ => literal.push(ch),
        }
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
pub fn lex(s: &str) -> Result<Vec<Token>, String> {
    let chars:Vec<char> = s.chars().collect();
    let mut pos = 0usize;
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
        while pos < chars.len() {
            let ch = chars[pos];
            match ch {
                '0'...'9'|'.' => {
                    let number = try!(lex_number(&chars, &mut pos));
                    push_tok(Token::Number(number));
                },
                'a'...'z'|'A'...'Z'|'_' => push_tok(Token::Identifier(lex_identifier(&chars, &mut pos))),
                '"' => push_tok(Token::String(try!(lex_string(&chars, &mut pos)))),
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

                // comments are handled in this block
                '/' => {
                    match peek_at_next_char(&chars, pos) {
                        Some('*') => {
                            // TODO: handle comment until a `*/` symbol
                        },

                        Some('/') => {
                            // comment till the end of the line
                            while {
                                pos += 1;
                                let ch = chars[pos];
                                ch != '\n'
                            }{};
                        },

                        _ => push_tok(Token::Operator(Operator::Divide)),
                    }
                },

                _ => return Err(format!("unexpected character '{}'", ch)),
            }
            pos += 1;
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comma_lparen_lbrace() {
        assert_eq!(lex(", ( {").unwrap(), vec![Token::Comma, Token::LParen, Token::LBrace]);
    }

    #[test]
    fn unexpected_character() {
        assert!(lex("$").is_err());
        assert!(lex("@").is_err());
    }

    #[test]
    fn numbers() {
        assert_eq!(lex("123").unwrap(), vec![Token::Number(Number::Int(123))]);
        assert_eq!(lex("12.3").unwrap(), vec![Token::Number(Number::Float(12.3))]);
        assert_eq!(lex("012").unwrap(), vec![Token::Number(Number::Int(12))]);
        assert_eq!(lex("0120}").unwrap(), vec![Token::Number(Number::Int(120)), Token::RBrace]);
    }

    #[test]
    fn identifiers() {
        assert_eq!(lex("int ident1, _ident2;").unwrap(),
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
        assert_eq!(lex(r##""\n\\\"""##).unwrap(), vec![Token::String("\n\\\"".to_string())]);
        assert!(lex("\"hello ").is_err());
        assert!(lex("\"hello \\").is_err());
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

        let tokens = lex(simple_program).unwrap();
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
                    Token::Number(Number::Int(0)),
                    Token::Semicolon,
                Token::RBrace,
            ]
        );
    }
}

