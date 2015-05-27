#![allow(unused_imports)]

use std;
use lexer::Token;
use lexer::Operator;
use lexer::Number;

use ast::*;

/**
 * TODO
 */
fn parse_expression(tokens: &Vec<Token>, pos: &mut usize) -> Result<Expression, String> {
    Err("unimplemented".to_string())
}


/**
 * Parse declarations such as
 *
 * ```
 * const int b = 10;
 * int *a = &b;
 * int c[10] = { 0 };
 * unsigned char d, e;
 * ```
 *
 * Into a valid Declaration struct. The rule for this parse looks like
 *
 * ```
 * ident+ (asterisk+ ident)? ((comma asterisk* ident)*|(equals <expression>)?)
 * ```
 *
 * A handcrafted parser may not be the most understandable way to build this construct :)
 */
fn parse_type(tokens: &Vec<Token>, pos: &mut usize) -> Result<Type, String> {
    Err("unimplemented".to_string())
}

/**
 * Look for function declarations of the form
 *
 * ```
 * void do_something(int a) {}
 * ```
 *
 * And variable declarations of the form
 *
 * ```
 * int a;
 * ```
 */
fn parse_program(tokens: &Vec<Token>, pos: &mut usize) -> Result<Program, String> {
    /*
    let globals = vec![];
    let functions = vec![];
    */

    loop {
        let tok = &((*tokens)[*pos]);
        match tok {
            &Token::Identifier(ref id) => {
                let _type = parse_type(tokens, pos);
            },
            _ => {
                return Err(format!("unexpected token {:?}", tok));
            },
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    if tokens.len() == 0 {
        return Err("parser: no tokens provided".to_string())
    }

    let mut pos = 0usize;
    let root_ast_node = try!(parse_program(&tokens, &mut pos));
    if pos != tokens.len() {
        return Err("parser: continuation past end of input".to_string())
    }
    Ok(root_ast_node)
}

