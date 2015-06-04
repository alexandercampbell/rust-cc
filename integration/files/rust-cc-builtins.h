/*
 * rust-cc-builtins.h
 *
 * Define the compiler builtins of rust-cc as standard C functions and macros.
 * This is so the same files can be compiled with rust-cc and clang or gcc.
 */

#ifndef RUST_CC_BUILTINS_H_
#define RUST_CC_BUILTINS_H_

#include <stdio.h>

// write_int will be a compiler builtin for us, but in standard C, we'd have to
// use something like dprintf. This is just because I'm too lazy to implement
// all of fprintf over again.
#define write_int(file_descriptor, integer) \
	dprintf(file_descriptor, "%d\n", integer)

#endif // RUST_CC_BUILTINS_H_

