
use ast;

mod context;
mod build;
pub mod lexer;

use parser::context::Context;
use parser::lexer::Token;

/**
 * Parse a series of Tokens into a complete Program AST. No evaluation or optimization is done
 * during this phase.
 */
pub fn parse(tokens: Vec<Token>) -> Result<ast::Program, String> {
    let mut context = Context::new(tokens);
    build::program(&mut context)
}

/**
 * Parse a program string directly by first lexing the tokens and then passing them to the parser.
 */
#[allow(unused)]
pub fn parse_str(s: &str) -> Result<ast::Program, String> {
    use parser::lexer::lex;
    let tokens = try!(lex(s));
    parse(tokens)
}

/**
 * Parse a single C expression in an Expression AST. No evaluation or optimization is done during
 * this phase.
 */
#[allow(unused)]
pub fn parse_expr(tokens: Vec<Token>) -> Result<ast::Expression, String> {
    let mut context = Context::new(tokens);
    let expr = try!(build::expression(&mut context));
    if !context.is_exhausted() {
        return Err(format!("tokens remained after parsing expression"))
    }
    Ok(expr)
}

/**
 * Parse an expression string directly by first lexing the tokens and then passing them to the
 * parser.
 */
#[allow(unused)]
pub fn parse_expr_str(s: &str) -> Result<ast::Expression, String> {
    use parser::lexer::lex;
    let tokens = try!(lex(s));
    parse_expr(tokens)
}

#[cfg(test)]
mod test {
    use super::*;
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

    #[test]
    fn empty_function_call() {
        let expr = parse_expr_str("hello()").unwrap();
        assert_eq!(expr, Expression::FunctionCall{
            name: "hello".to_string(),
            args: vec![],
        });
    }

    #[test]
    fn function_call_with_parameters() {
        let expr = parse_expr_str(r##"concatenate_these_strings("alpha", "beta", "charlie", "delta")"##).unwrap();
        assert_eq!(expr, Expression::FunctionCall{
            name: "concatenate_these_strings".to_string(),
            args: vec![
                Expression::String("alpha".to_string()),
                Expression::String("beta".to_string()),
                Expression::String("charlie".to_string()),
                Expression::String("delta".to_string()),
            ],
        });
    }


    #[test]
    fn operator_precedence() {
        let expr = parse_expr_str("1 - 2 * 3 + 4").unwrap();

        assert_eq!(expr,
            Expression::BinaryOp(
                box Expression::Number(Number::Int(1)),
                BinaryOp::Subtract,
                box Expression::BinaryOp(
                    box Expression::BinaryOp(
                        box Expression::Number(Number::Int(2)),
                        BinaryOp::Multiply,
                        box Expression::Number(Number::Int(3)),
                    ),
                    BinaryOp::Add,
                    box Expression::Number(Number::Int(4)),
                ),
            )
        );
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

