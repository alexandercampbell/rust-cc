/*
 * hello_addition.c
 *
 * Demonstrate main(), variable declarations, addition, multiplication, and
 * expressions inside functions.
 */

#include "rust-cc-builtins.h"

// `int main()` will be our initially supported signature for main. In the
// future we may add support for other signatures such as
//
//	void main()
//	int main(int argc, char *argv[])
//	int main(int argc, char **argv)
//
int main() {
	int a;
	int b;
	int file_descriptor = 1;

	a = 10 + 2;
	b = a * 10;

	write_int(file_descriptor, a);
	write_int(file_descriptor, b);
	write_int(file_descriptor, a + b);

	return 0;
}

