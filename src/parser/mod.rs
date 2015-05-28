
use lexer::Token;
use ast::*;

mod context;
use self::context::Context;

mod build;

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
    let root_ast_node = try!(build::program(&mut context));
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

    /**
     * Test a very simple program that merely declares a constant integer.
     */
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

