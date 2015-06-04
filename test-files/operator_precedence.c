/*
 * operator_precedence.c
 *
 * Print out the results of some complicated mathematical expressions. This file
 * will include examples of bitwise operators, logical operators, unusual
 * operators like "sizeof", unary operators, and other operators.
 */

#include "rust-cc-builtins.h"

int main() {
	write_int(1, 2 * 3 + 4);
	write_int(1, 1 + 2 * 3 + 4);
	write_int(1, 2 - 3 / 4 + 7 + 3 * 2 - 2);
}

