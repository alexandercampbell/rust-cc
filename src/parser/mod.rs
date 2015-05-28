
use lexer::Token;
use ast::*;

mod context;
use self::context::Context;

/**
 * Parse declarations such as
 *
 * ```
 * const int b = 10;
 * int *a = &b;
 * int c[10] = { 0 };
 * unsigned char d, e;
 * int f = *a * b;
 * ```
 *
 * Into Declaration structs. The rule for this parse looks like
 *
 * ```
 * ident+ (asterisk+ ident)? ((comma asterisk* ident)*|(equals <expression>)?)
 * ```
 *
 * A handcrafted parser may not be the most understandable way to build this construct :)
 */
fn parse_declaration(context: &mut Context) -> Result<Declaration, String> {
    Err("unimplemented".to_string())
}

/**
 * Look for function declarations of the form
 *
 * ```
 * void do_something(int a) {}
 * void do_something(int a); // forward declaration
 * ```
 *
 * And variable declarations of the form
 *
 * ```
 * int a;
 * ```
 */
fn parse_program(context: &mut Context) -> Result<Program, String> {
    let mut program = Program{
        globals: vec![],
        functions: vec![],
    };

    loop {
        match context.peek() {
            Some(Token::Identifier(id)) => {
                // first, try to parse it as a declaration
                let declaration = parse_declaration(context);
                if declaration.is_ok() {
                    program.globals.push(declaration.unwrap());
                    continue;
                }

                // second, try to parse it as a function
                // TODO
            },
            Some(tok) => {
                return Err(format!("unexpected token {:?}", tok));
            },
            None => {
                return Err("I'd return a Program but I haven't been implemented".to_string());
            }
        }
    }
}

/**
 * Parse a series of Tokens into a complete Program AST. No evaluation or optimization is done
 * during this phase.
 */
pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    if tokens.len() == 0 {
        return Err("parser: no tokens provided".to_string())
    }

    let mut pos = 0usize;
    let tokens_len = tokens.len();
    let mut context = context::Context::new(tokens);
    let root_ast_node = try!(parse_program(&mut context));
    if pos != tokens_len {
        return Err("parser: continuation past end of input".to_string())
    }
    Ok(root_ast_node)
}

#[cfg(test)]
mod test {
    use super::parse;
    use ast::*;
    use lexer::lex;

    fn constant_declaration() {
        let tokens = lex("const int a;").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program, Program{
            globals:   vec![
                Declaration{
                    variable: "a".to_string(),
                    _type: Type{
                        base_name:      "int".to_string(),
                        modifiers:      vec!["const".to_string()],
                        length:         None,
                        pointer_levels: 0,
                    },
                    initial_value: None,
                },
            ],
            functions: vec![],
        });
    }
}

