/*!
 * rust-cc
 * =======
 *
 * rust-cc will be a C compiler written in Rust.
 *
 * rust-cc is a cross between learning project and proof of concept. Do not use this program for C
 * development.
 *
 * maintainer
 * ----------
 *
 * - [Alexander Campbell](mailto:alexanderhcampbell@gmail.com)
 *
 */
#![feature(box_syntax)]

pub mod ast;
mod checker;
pub mod interpreter;
pub mod parser;
pub mod source;
pub mod util;

