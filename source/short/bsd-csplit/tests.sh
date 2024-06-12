#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

# Function to test output files content against expected content
# Usage: test_content "expected content" output_files...
test_content() {
    expected="$1"
    shift
    for file in "$@"; do
        if ! diff <(echo -n "$expected") "$file" > /dev/null; then
            echo "Test failed: Content of $file does not match expected content."
            return 1
        fi
    done
    return 0
}

# Setup test environment
mkdir -p test_files
cd test_files

# Cleanup function to remove test files
cleanup() {
    rm -f xx* infile testfile*
}
# trap cleanup EXIT

# Prepare an input file
printf "Line 1\nLine 2\nLine 3\nLine 4\nLine 5" > infile

# Test splitting by line number
../csplit infile 3 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 1.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 1.2 PASSED"

# Test splitting by regex
rm xx00 xx01
../csplit infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 2.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 2.2 PASSED"

# Test suffix length option
../csplit -n 4 infile 3 > /dev/null
[ -f xx0000 ] && echo "Test 4 PASSED"

# Test prefix option
../csplit -f testfile infile 3 > /dev/null
[ -f testfile00 ] && echo "Test 5 PASSED"

# Test error handling for invalid line number
if ! ../csplit infile 100 2>/dev/null; then
    echo "Test 6 PASSED"
fi

# Test error handling for regex with no match
if ! ../csplit infile /NoMatch/ 2>/dev/null; then
    echo "Test 7 PASSED"
fi

# Exiting
cleanup
cd ..
echo "All tests done."

# Code coverage
gcov ./csplit.c
