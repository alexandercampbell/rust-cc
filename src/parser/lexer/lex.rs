
use ast;
use parser::lexer::Token;

/**
 * Retrieve the next `char` after `pos` if possible.
 */
pub fn peek_at_next_char(chars: &Vec<char>, pos: usize) -> Option<char> {
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
pub fn number(chars: &Vec<char>, pos: &mut usize) -> Result<Token, String> {
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
        if literal == "." {
            return Ok(Token::Period);
        }

        let f = match literal.parse::<f64>() {
            Ok(f) => f,
            Err(_) => return Err(format!("bad floating point literal '{}'", literal)),
        };
        Ok(Token::Number(ast::Number::Float(f)))
    } else {
        let i = match literal.parse::<i64>() {
            Ok(i) => i,
            Err(_) => return Err(format!("bad integer literal '{}'", literal)),
        };
        Ok(Token::Number(ast::Number::Int(i)))
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
pub fn identifier(chars: &Vec<char>, pos: &mut usize) -> String {
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
pub fn string(chars: &Vec<char>, pos: &mut usize) -> Result<String, String> {
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
