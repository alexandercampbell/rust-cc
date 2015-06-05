/*!
 * This module contains the functions that build ASTs from Tokens.
 */

use parser::lexer::Token;
use parser::lexer::Operator;
use ast::*;

use super::context::Context;

/**
 * This function is named "atom" because I can't remember what the fundamental AST nodes are
 * called. Terminals, maybe?
 *
 * Either way, this function returns one of the fundamental units of the AST (sometimes). The
 * exception to this rule is function calls, which obviously can contain full expressions as
 * arguments.
 */
fn atom(context: &mut Context) -> Result<Expression, String> {
    match context.next() {
        Some(Token::String(s)) => Ok(Expression::String(s)),
        Some(Token::Character(ch)) => Ok(Expression::Character(ch)),
        Some(Token::Number(n)) => Ok(Expression::Number(n)),
        Some(Token::Identifier(id)) => {
            match context.peek() {
                Some(Token::LParen) => {
                    /*
                     * Function call on the identifier.
                     */
                    context.next(); // consume the opening LParen

                    let function_name = id;
                    let mut arguments: Vec<Expression> = vec![];

                    match context.peek() {
                        Some(Token::RParen) => {
                            // no arguments to function call
                            context.next(); // consume the Token::RParen
                            Ok(Expression::FunctionCall{name: function_name, args: vec![]})
                        }

                        _ => {
                            let first_arg = try!(expression(context));
                            arguments.push(first_arg);

                            loop {
                                match context.next() {
                                    Some(Token::RParen) => break,
                                    Some(Token::Comma) => arguments.push(try!(expression(context))),

                                    Some(tok) => return Err(format!(
                                            "unexpected token {:?} while parsing arguments to function {:?}",
                                            tok, function_name)),

                                    None => return Err(format!(
                                            "unexpected EOF while parsing arguments to function {:?}",
                                            function_name)),
                                }
                            }

                            Ok(Expression::FunctionCall{name: function_name, args: arguments})
                        }
                    }
                },

                _ => Ok(Expression::Variable(id))
            }
        }
        Some(token) => Err(format!("expected atom token, got {:?}", token)),
        None => Err("expected atom token".to_string()),
    }
}

/**
 * Parse one of the unary operators: `+`, `-`, `*`, or `&`.
 */
fn unary_op(context: &mut Context) -> Result<Expression, String> {
    match context.peek() {
        Some(Token::Operator(lexer_op @ _)) => {
            let parser_op = match lexer_op {
                Operator::Add => UnaryOp::DontNegate,
                Operator::Subtract => UnaryOp::Negate,
                Operator::Reference => UnaryOp::Reference,
                Operator::Asterisk => UnaryOp::Dereference,

                // not a unary op.
                _ => return atom(context),
            };

            context.next(); // consume token
            let rhs = try!(atom(context));
            Ok(Expression::UnaryOp(parser_op, box rhs))
        },

        _ => atom(context),
    }
}

/**
 * Convert from a lexer operator to a parser BinaryOp.
 *
 * This function is not possible for a generic case because the lexer operators have different
 * meanings depending on the context. For example, `*` can mean either "dereference" or "multiply".
 * However, this function is unambiguous because it only converts to a BinaryOp.
 */
fn lexer_op_to_parser_op(lexer_op: Operator) -> Option<BinaryOp> {
    Some(match lexer_op {
        Operator::Asterisk => BinaryOp::Multiply,
        Operator::Add => BinaryOp::Add,
        Operator::Subtract => BinaryOp::Subtract,
        Operator::Divide => BinaryOp::Divide,
        Operator::Modulo => BinaryOp::Modulo,
        Operator::And => BinaryOp::And,
        Operator::Or => BinaryOp::Or,
        Operator::Assign => BinaryOp::Assign,
        _ => return None,
    })
}

/**
 * Construct an operator node with the only operators allowed coming from `allowed_operators`.
 * `build_subtree` refers to the next-looser precedence level.
 */
fn binary_operator_helper(
    context: &mut Context,
    allowed_operators: &'static [BinaryOp],
    build_subtree: fn(&mut Context) -> Result<Expression, String>
) -> Result<Expression, String> {

    let lhs_node = try!(build_subtree(context));
    let operator;

    // Look for an operator token, if one exists.
    match context.peek() {
        None => return Ok(lhs_node),
        Some(Token::Operator(tok)) => {
            let parser_op = lexer_op_to_parser_op(tok);
            match parser_op {
                Some(ref o) if allowed_operators.contains(&o) => {
                    operator = o.clone();
                    context.next();
                },
                _ => return Ok(lhs_node),
            }
        },
        _ => return Ok(lhs_node),
    }

    let rhs_node = try!(binary_operator_helper(context, allowed_operators, build_subtree));
    Ok(Expression::BinaryOp(
        box lhs_node,
        operator,
        box rhs_node,
    ))
}

fn multiplication(context: &mut Context) -> Result<Expression, String> {
    static OPERATORS: [BinaryOp; 2] = [BinaryOp::Multiply, BinaryOp::Divide];
    binary_operator_helper(context, &OPERATORS, unary_op)
}

fn addition(context: &mut Context) -> Result<Expression, String> {
    static OPERATORS: [BinaryOp; 2] = [BinaryOp::Add, BinaryOp::Subtract];
    binary_operator_helper(context, &OPERATORS, multiplication)
}

fn boolean_ops(context: &mut Context) -> Result<Expression, String> {
    static OPERATORS: [BinaryOp; 2] = [BinaryOp::And, BinaryOp::Or];
    binary_operator_helper(context, &OPERATORS, addition)
}

/**
 * Parse a single expression. Many things in C are expressions, including declarations and
 * assignments.
 */
pub fn expression(context: &mut Context) -> Result<Expression, String> {
    boolean_ops(context)
}

/**
 * Parse a statement.
 */
fn statement(context: &mut Context) -> Result<Statement, String> {
    let expr = try!(expression(context));
    match context.next() {
        Some(Token::Semicolon) => Ok(Statement::Expression(expr)),
        Some(token) => Err(format!("unexpected token {:?} after expression", token)),
        None => Err(format!("expected semicolon after statement")),
    }
}

/**
 * Parse a block of statements. This may be either a single statement or a series of statements
 * enclosed in curly braces `{}`.
 *
 * The astute among you may notice that this is very similar to argument parsing :)
 */
fn statement_block(context: &mut Context) -> Result<Vec<Statement>, String> {
    match context.peek() {
        Some(Token::LBrace) => {
            context.next();
            let mut statements = vec![];
            loop {
                match context.peek() {
                    Some(Token::RBrace) => {
                        context.next(); // consume the closing paren
                        return Ok(statements);
                    },

                    Some(_) => {
                        let statement = try!(statement(context));
                        statements.push(statement);
                    },

                    None => return Err("unterminated statement block".to_string()),
                }
            }
        },

        Some(_) => {
            let statement = try!(statement(context));
            Ok(vec![statement])
        },

        None => return Err("expected statement in statement block".to_string()),
    }
}

/**
 * Parse function definitions such as
 *
 * ```c
 *      void say_hello() {}
 *      const int number_of_processes() { return 5; }
 * ```
 *
 * into the appropriate ast::Function structures.
 *
 * NOTE: This function assumes that the type declaration has already been parsed, up to and
 * including the left paren of the argument list.
 */
fn function_definition(context: &mut Context, signature: Declaration) -> Result<Function, String> {
    /*
     * Argument parsing
     */
    let mut arguments: Vec<Declaration> = vec![];

    match context.peek() {
        Some(Token::RParen) => {
            // A RParen immediately means the arguments list is empty; the function signature looks
            // like this (no arguments):
            //
            //      int my_function()
            //

            context.next(); // Consume the Token::RParen
        },

        _ => {
            // Parse arguments (in this branch, we know there is at least one argument ready to be
            // parsed).
            let first_arg = try!(declaration(context));
            arguments.push(first_arg);

            loop {
                match context.next() {
                    Some(Token::RParen) => break,
                    Some(Token::Comma)  => arguments.push(try!(declaration(context))),

                    Some(tok) => return Err(format!("unexpected token {:?} while parsing function argument list", tok)),
                    None      => return Err("unexpected EOF when parsing function argument list".to_string()),
                }
            }
        },
    }

    /*
     * Body parsing
     */
    let statements = try!(statement_block(context));

    Ok(Function{
        name:           signature.name,
        arguments:      arguments,
        return_type:    signature._type,
        statements:     statements,
    })
}

/**
 * Parse declarations such as
 *
 * ```c
 *      const int b
 *      int *a
 *      int c[10]
 *      unsigned char d, e
 *      int f
 * ```
 *
 * into ast::Declaration structs. The rule for this parse looks something like
 *
 * ```c
 *      ident+ (asterisk+ ident)? ((comma asterisk* ident)*)
 * ```
 *
 * A handcrafted parser may not be the most understandable way to build this construct :)
 *
 */
fn declaration(context: &mut Context) -> Result<Declaration, String> {

    //
    // parse identifiers until we see
    //
    // 1. An asterisk. This means **type** definition is essentially over and a variable name is
    //    next. Note that it is technically possible for multiple asterisks to appear.
    // 2. A token we don't recognize. This means the variable declaration is over. Return what we
    //    have so far.
    //

    let mut identifiers: Vec<String> = vec![];

    let first_token = context.next();
    match first_token {
        Some(Token::Identifier(ident)) => identifiers.push(ident),
        _ => return Err("expected identifier at beginning of declaration".to_string()),
    };

    loop {
        match context.next() {
            // Keep pushing identifiers until we hit something else. This is the only match arm
            // that will actually continue the loop.
            Some(Token::Identifier(ident)) => identifiers.push(ident),

            // This is a case where an asterisk interrupts the stream of tokens. This tells us some
            // important information about the identifiers we just grabbed. Example:
            //
            //      const int *b;
            //
            // We can easily infer that the previous token (int) was the base type, the tokens
            // before that were modifiers, and the token immediately after the asterisk is the
            // variable name. The last part there is not always true; sometimes multiple asterisks
            // are chained together as in this declaration:
            //
            //      const int ***c;
            //
            Some(Token::Operator(Operator::Asterisk)) => {
                let base_name = identifiers.pop().unwrap();
                let modifiers = identifiers;
                let mut pointer_levels = 1;
                let variable_name: String;

                loop {
                    // In this loop, we're looking for either another asterisk or an identifier
                    // (the variable name). This is only a loop because the number of asterisks can
                    // vary freely.
                    match context.next() {
                        Some(Token::Operator(Operator::Asterisk)) => pointer_levels += 1,
                        Some(Token::Identifier(string)) => {
                            variable_name = string;
                            break;
                        }
                        _ => return Err("expected either variable name or asterisk after asterisk in declaration".to_string()),
                    }
                }

                // TODO: support for multiple comma-separated declarations and array declarations
                // (such as [10]).
                return Ok(Declaration{
                    _type: Type{
                        modifiers:      modifiers,
                        base_name:      base_name,
                        pointer_levels: pointer_levels,
                        length:         None,
                    },
                    name: variable_name,
                });
            }

            // This is a very simple case: a series of identifiers followed by a semicolon or comma
            // or something like that. We don't really care what the next token is because we might
            // be inside a function's argument list or part of a global variable declaration.
            // Example:
            //
            //      const unsigned long a;
            //
            _ => {
                context.step_back(); // don't want to absorb the next token if it exists

                if identifiers.len() < 2 {
                    return Err("expected at least two identifiers before semicolon".to_string());
                }

                // the following comments assume a declaration such as
                //
                //      const unsigned int my_integer;
                //
                let name = identifiers.pop().unwrap();      // my_integer
                let base_name = identifiers.pop().unwrap(); // int
                let modifiers = identifiers;                // [const, unsigned]

                return Ok(Declaration{
                    _type: Type{
                        base_name:      base_name,
                        modifiers:      modifiers,
                        length:         None,
                        pointer_levels: 0,
                    },
                    name: name,
                });
            },
        }
    }
}

/**
 * Look for function declarations of the form
 *
 * ```c
 * void do_something(int a) {}
 * void do_something(int a); // forward declaration
 * ```
 *
 * And variable declarations of the form
 *
 * ```c
 * int a;
 * ```
 */
// build::program is our only exported identifier from this module.
pub fn program(context: &mut Context) -> Result<Program, String> {
    let mut program = Program{
        globals: vec![],
        functions: vec![],
    };

    loop {
        match context.peek() {
            Some(Token::Identifier(_)) => {
                let declaration = try!(declaration(context));

                match context.next() {
                    // Global variable declaration without initialization.
                    //
                    //      int num_rows;
                    //
                    Some(Token::Semicolon) => {
                        program.globals.push(declaration);
                        continue;
                    },

                    // TODO: handle Operator::Assign in this block for global variable
                    // declarations like
                    //
                    //      const int NUM_ROWS = 100;
                    //
                    Some(Token::Operator(Operator::Assign)) => {},

                    // Function definition
                    Some(Token::LParen) => {
                        let function = try!(function_definition(context, declaration));
                        program.functions.push(function);
                    },

                    _ => return Err("expected semicolon after global variable declaration".to_string()),
                }
            },

            Some(tok)   => return Err(format!("unexpected token {:?}", tok)),
            None        => return Ok(program),
        }
    }
}

// TODO: theoretically, each builder function should have its own unit test.

