#
# This file compiles each ".c" file in the current directory with clang and
# rust-cc. Then, both versions are run, and their outputs are compared. Any
# differences in output are printed.
#

set -o pipefail

rm -r "test-output"
mkdir -p "test-output"

CLANG_EXEC="test-output/clang_executable.out"
CLANG_OUT="test-output/clang_output.txt"

cargo build
RUST_CC="../target/debug/rust-cc"
RUST_CC_OUT="test-output/rust_cc_output.txt"

for test_file in *.c ; do
    echo "Comparing outputs for file '$test_file'"
    (set -x; clang -o $CLANG_EXEC "$test_file")
    $CLANG_EXEC > $CLANG_OUT

    (set -x; $RUST_CC "$test_file" > $RUST_CC_OUT)

    DIFF=`diff "$CLANG_OUT" "$RUST_CC_OUT"`
    if [ -z $DIFF ]; then
        echo "pass: output was identical"
    else
        echo "FAIL:"
        echo "$DIFF"
    fi
done

# : vim: set et sw=4 ts=4 :

