
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

/**
 * Parse a string directly by first lexing the tokens and then passing them to the parser.
 */
#[allow(unused)]
pub fn parse_str(s: &str) -> Result<ast::Program, String> {
    use lexer::lex;
    let tokens = try!(lex(s));
    parse(tokens)
}

#[cfg(test)]
mod test {
    use super::{parse,parse_str};
    use ast::*;

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
        let program = parse_str("const int a;").unwrap();
        assert_eq!(program, Program{
            globals: vec![
                Declaration{
                    name: "a".to_string(),
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
        let program = parse_str("unsigned short **pointer;").unwrap();
        assert_eq!(program, Program{
            globals: vec![
                Declaration{
                    name: "pointer".to_string(),
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

    #[test]
    fn void_function() {
        let program = parse_str(r##"
                         void hello() {}
                         "##).unwrap();

        assert_eq!(program, Program{
            globals: vec![],
            functions: vec![
                Function{
                    name: "hello".to_string(),
                    return_type: Type{
                        base_name:      "void".to_string(),
                        modifiers:      vec![],
                        length:         None,
                        pointer_levels: 0,
                    },
                    arguments: vec![],
                    statements: vec![],
                },
            ],
        });
    }

    /**
     * Test a simple function definition with a single statement inside.
     */
    #[test]
    fn function_definition() {
        let program = parse_str(r##"
                         inline const void get_num_cores(int *a) {
                            -10;
                         }
                         "##).unwrap();

        assert_eq!(program, Program{
            globals: vec![],
            functions: vec![
                Function{
                    name: "get_num_cores".to_string(),
                    return_type: Type{
                        base_name:      "void".to_string(),
                        modifiers:      vec!["inline".to_string(), "const".to_string()],
                        length:         None,
                        pointer_levels: 0,
                    },
                    arguments: vec![
                        Declaration{
                            name: "a".to_string(),
                            _type: Type{
                                base_name:      "int".to_string(),
                                modifiers:      vec![],
                                length:         None,
                                pointer_levels: 1,
                            }
                        },
                    ],
                    statements: vec![
                        Statement::Expression(
                            Expression::UnaryOp(
                                UnaryOp::Negate,
                                box Expression::Number(Number::Int(10)),
                            )
                        ),
                    ],
                },
            ],
        });
    }
}

