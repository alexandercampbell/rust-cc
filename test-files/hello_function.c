/*
 * hello_function.c
 *
 * Demonstrate function calls and variable shadowing.
 */

#include "rust-cc-builtins.h"

int count;

void increment_count() { count = count + 1; }
void decrement_count() { count = count - 1; }
int count_tripled()    { return count * 3; }

void count_shadower() {
	int count = 5;
	count = count + 5;
}

int main() {
	int file_descriptor = 0; // stdout

	// initialize count
	count = 0;
	write_int(file_descriptor, count);

	// increment and print
	increment_count();
	write_int(file_descriptor, count);

	// decrement and print
	decrement_count();
	write_int(file_descriptor, count);

	// call a function that has a variable that shadows `count`
	count_shadower();
	write_int(file_descriptor, count);
}

