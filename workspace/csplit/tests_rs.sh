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
../target/debug/short infile 3 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 1.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 1.2 PASSED"

# Test splitting by regex
rm xx00 xx01
../target/debug/short infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 2.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 2.2 PASSED"

# Test suffix length option
../target/debug/short -n 4 infile 3 > /dev/null
[ -f xx0000 ] && echo "Test 3 PASSED"

# Test prefix option
../target/debug/short -f testfile infile 3 > /dev/null
[ -f testfile00 ] && echo "Test 4 PASSED"

# Test error handling for invalid line number
if ! ../target/debug/short infile 100 2>/dev/null; then
    echo "Test 5 PASSED"
else
    echo "Test 5 FAILED"
fi

# Test error handling for regex with no match
if ! ../target/debug/short infile /NoMatch/ 2>/dev/null; then
    echo "Test 6 PASSED"
else
    echo "Test 6 FAILED"
fi

rm -f xx*
../target/debug/short -s infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 7.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 7.2 PASSED"

# Test keeping files with no output
rm -f xx*
../target/debug/short -k infile 100 2> /dev/null
[ -f xx00 ] && echo "Test 8 PASSED"

# Test custom numeric suffix starting
rm -f testfile*
../target/debug/short -f testfile infile 3 > /dev/null
[ -f testfile00 ] && echo "Test 9 PASSED"

if ! ../target/debug/short infile --definitely-invalid 2>/dev/null; then
    echo "Test 10 PASSED"
fi

rm -f xx*
printf "1\n2\n3\n4\n5\n" | ../target/debug/short - 3 > /dev/null
test_content $'1\n2\n' xx00 && echo "Test 11.1 PASSED"
test_content $'3\n4\n5\n' xx01 && echo "Test 11.2 PASSED"

# Test for specific line number
rm -f xx*
../target/debug/short infile 2 > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 12.1 PASSED"
test_content $'Line 2\nLine 3\nLine 4\nLine 5' xx01 && echo "Test 12.2 PASSED"

# Test for offset (positive and negative)
rm -f xx*
../target/debug/short infile /Line\ 2/+1 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 13.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 13.2 PASSED"

rm -f xx*
../target/debug/short infile /Line\ 3/-1 > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 14.1 PASSED"
test_content $'Line 2\nLine 3\nLine 4\nLine 5' xx01 && echo "Test 14.2 PASSED"

# test for multiple offsets
rm -f xx*
../target/debug/short infile /Line\ 2/+1 /Line\ 4/-1 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 15 PASSED"

# Test for repeating splits
rm -f xx*
../target/debug/short infile 2 "{2}" > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 16.1 PASSED"
test_content $'Line 2\nLine 3\n' xx01 && echo "Test 16.2 PASSED"
test_content $'Line 4\nLine 5' xx02 && echo "Test 16.3 PASSED"

# test invalid split count
if ! ../target/debug/short infile 2 "{-1}" 2>/dev/null; then
    echo "Test 17 PASSED"
else
    echo "Test 17 FAILED"
fi

# Use csplit with a large value for `-n` to specify the suffix length
if ! ../target/debug/short -f "output_file_prefix" -n 19 infile "%pattern%" "{*}" 2>/dev/null; then
    echo "Test 18 PASSED"
else
    echo "Test 18 FAILED"
fi

# Test for option --prefix
if ../target/debug/short -f custom_prefix infile 2 > /dev/null; then
    [ -f custom_prefix00 ] && echo "Test 19 PASSED"
else
    echo "Test 19 FAILED"
fi

rm -f custom_prefix*

# Test for keeping files with -k even on error
if ! ../target/debug/short -k infile /NoMatch/ 2>/dev/null; then
    [ -f xx00 ] && echo "Test 20 PASSED"
else
    echo "Test 20 FAILED"
fi

# Test no match
if ! ../target/debug/short infile /NoMatch/ 2>/dev/null; then
    echo "Test 21 PASSED"
else
    echo "Test 21 FAILED"
fi

# Test for option --suppress-matched
../target/debug/short -s infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 22 PASSED"

# Test max suffix length 
if ! ../target/debug/short -n 1000000000000000000000 infile 3 2>/dev/null; then
    echo "Test 23 PASSED"
else
    echo "Test 23 FAILED"
fi

# Test too long name
if ! ../target/debug/short -n 100000000000000 infile 3 2>/dev/null; then
    echo "Test 24 PASSED"
else
    echo "Test 24 FAILED"
fi

# Test invalid flags and options
if ! ../target/debug/short -z infile 3 2>/dev/null; then
    echo "Test 25 PASSED"
else
    echo "Test 25 FAILED"
fi

# test bad repitition count 
if ! ../target/debug/short infile 2 "{-1}" 2>/dev/null; then
    echo "Test 26 PASSED"
else
    echo "Test 26 FAILED"
fi

# test non existent file
if ! ../target/debug/short non_existent_file 2 2>/dev/null; then
    echo "Test 27 PASSED"
else
    echo "Test 27 FAILED"
fi

# trigger ENAMETOOLONG error
long_prefix=$(printf '%*s' 255 | tr ' ' 'a')

# Attempt to use csplit with a long prefix. Adjust the '2' if necessary to control the suffix length.
if ! ../target/debug/short -f="$long_prefix" infile 2>/dev/null; then
    echo "Test 28 PASSED"
else
    echo "Test 28 FAILED"
fi

# Trigger overflow
if ! ../target/debug/short infile 1000000000000000000000 2>/dev/null; then
    echo "Test 29 PASSED"
else
    echo "Test 29 FAILED"
fi

# invalid regex pattern
if ! ../target/debug/short infile /Line\ 2 2>/dev/null; then
    echo "Test 30 PASSED"
else
    echo "Test 30 FAILED"
fi

# trigger invalid 
if ! ../target/debug/short infile \\/Line\ 2 2>/dev/null; then
    echo "Test 31 PASSED"
else
    echo "Test 31 FAIED"
fi

# trigger invalid 
if ! ../target/debug/short infile %%/Line\ 2 2>/dev/null; then
    echo "Test 32 PASSED"
else
    echo "Test 32 FAILED"
fi

# test invalid line number
if ! ../target/debug/short infile -1 2>/dev/null; then
    echo "Test 33 PASSED"
else
    echo "Test 33 FAILED"
fi


# Exiting
cleanup
cd ..
echo "All tests done."

# Code coverage
gcov ./src/main.rs