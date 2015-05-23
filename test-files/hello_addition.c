/*
 * hello_addition.c
 *
 * Demonstrate main(), variable declarations, addition, multiplication, and
 * expressions inside functions.
 */

#include <stdio.h>

// write_int will be a compiler builtin for us, but in standard C, we'd have to
// use something like dprintf. This is just because I'm too lazy to implement
// all of fprintf over again.
#define write_int(file_descriptor, integer) dprintf(file_descriptor, "%d\n", integer)

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
	int file_descriptor = 0;

	a = 10 + 2;
	b = a * 10;

	write_int(file_descriptor, a);
	write_int(file_descriptor, b);
	write_int(file_descriptor, a + b);

	return 0;
}

