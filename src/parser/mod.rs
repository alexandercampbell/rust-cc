
use lexer::Token;
use ast;

mod context;
use self::context::Context;

mod build;

/**
 * Parse a series of Tokens into a complete Program AST. No evaluation or optimization is done
 * during this phase.
 */
pub fn parse(tokens: Vec<Token>) -> Result<ast::Program, String> {
    let mut context = context::Context::new(tokens);
    build::program(&mut context)
}

#[cfg(test)]
mod test {
    use super::parse;
    use ast::*;
    use lexer::lex;

    #[test]
    fn empty_program() {
        let program = parse(vec![]).unwrap();
        assert_eq!(program, Program{
            globals:    vec![],
            functions:  vec![],
        });
    }

    /**
     * Test a very simple program that merely declares a constant integer.
     */
    #[test]
    fn constant_declaration() {
        let tokens = lex("const int a;").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program, Program{
            globals: vec![
                Declaration{
                    variable: "a".to_string(),
                    _type: Type{
                        base_name:      "int".to_string(),
                        modifiers:      vec!["const".to_string()],
                        length:         None,
                        pointer_levels: 0,
                    },
                },
            ],
            functions: vec![],
        });
    }

    /**
     * Test a pointer-pointer declaration.
     */
    #[test]
    fn pointer_pointer() {
        let tokens = lex("unsigned short **pointer;").unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program, Program{
            globals: vec![
                Declaration{
                    variable: "pointer".to_string(),
                    _type: Type{
                        base_name:      "short".to_string(),
                        modifiers:      vec!["unsigned".to_string()],
                        length:         None,
                        pointer_levels: 2,
                    },
                },
            ],
            functions: vec![],
        });
    }
}

