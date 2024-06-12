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
../short infile 3 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 1.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 1.2 PASSED"

# Test splitting by regex
rm xx00 xx01
../short infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 2.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 2.2 PASSED"

# Test suffix length option
../short -n 4 infile 3 > /dev/null
[ -f xx0000 ] && echo "Test 4 PASSED"

# Test prefix option
../short -f testfile infile 3 > /dev/null
[ -f testfile00 ] && echo "Test 5 PASSED"

# Test error handling for invalid line number
if ! ../short infile 100 2>/dev/null; then
    echo "Test 6 PASSED"
fi

# Test error handling for regex with no match
if ! ../short infile /NoMatch/ 2>/dev/null; then
    echo "Test 7 PASSED"
fi

rm -f xx*
if ! ../short -k infile 100 2>/dev/null; then
    echo "Test 8.1 PASSED"
fi
[ -f xx00 ] && echo "Test 8.2 PASSED"

rm -f xx*
if ! ../short infile 100 2>/dev/null; then
    echo "Test 9.1 PASSED"
fi
# file should not be created
[ ! -f xx00 ] && echo "Test 9.2 PASSED" || echo "Test 9.2 FAILED"

rm -f xx*
../short -s infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 8.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 8.2 PASSED"

# Test keeping files with no output
rm -f xx*
if ! ../short -k infile 100 2>/dev/null; then 
    [ -f xx00 ] && echo "Test 9.1 PASSED"
fi
# Test custom numeric suffix starting
# rm -f testfile* # failed
# ../short -f testfile -b "%03d.txt" infile 3 > /dev/null
# [ -f testfile002.txt ] && echo "Test 10 PASSED"

if ! ../short infile --definitely-invalid 2>/dev/null; then
    echo "Test 10 PASSED"
fi
rm -f xx*
printf "1\n2\n3\n4\n5\n" | ../short - 3 > /dev/null
test_content $'1\n2\n' xx00 && echo "Test 11.1 PASSED"
test_content $'3\n4\n5\n' xx01 && echo "Test 11.2 PASSED"

# Test for specific line number
rm -f xx*
../short infile 2 > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 12.1 PASSED"
test_content $'Line 2\nLine 3\nLine 4\nLine 5' xx01 && echo "Test 12.2 PASSED"

# Test for offset (positive and negative)
rm -f xx*
../short infile /Line\ 2/+1 > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 13.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx01 && echo "Test 13.2 PASSED"

rm -f xx*
../short infile /Line\ 3/-1 > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 14.1 PASSED" 
test_content $'Line 2\nLine 3\nLine 4\nLine 5' xx01 && echo "Test 14.2 PASSED" 

rm -f xx*
# test for multiple offsets
../short infile /Line\ 2/+1 /Line\ 4/-1 > /dev/null # failed this
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 15.1 PASSED"
test_content $'Line 3\nLine 4\nLine 5' xx02 && echo "Test 15.2 PASSED"
test_content $'' xx01 && echo "Test 15.3 PASSED"

# Test for repeating splits
rm -f xx*
../short infile 2 "{2}" > /dev/null
test_content $'Line 1\n' xx00 && echo "Test 16.1 PASSED" # failed this
test_content $'Line 2\nLine 3\n' xx01 && echo "Test 16.2 PASSED"
test_content $'Line 4\nLine 5' xx02 && echo "Test 16.3 PASSED"

# test invalid split count
if ! ../short infile 2 "{-1}" 2>/dev/null; then
    echo "Test 17 PASSED"
fi

# Use csplit with a large value for `-n` to specify the suffix length
if ! ../short -f "output_file_prefix" -n 19 infile "%pattern%" "{*}" 2>/dev/null; then
    echo "Test 18 PASSED"
fi

# Test for option --prefix
../short -f custom_prefix infile 2 > /dev/null
[ -f custom_prefix00 ] && echo "Test 19.1 PASSED"
[ -f custom_prefix01 ] && echo "Test 19.2 PASSED"
rm -f custom_prefix*

# Test for keeping files with -k even on error
if ! ../short -k infile /NoMatch/ 2>/dev/null; then
    [ -f xx00 ] && echo "Test 20 PASSED"
fi
rm -f xx*

# Test no match
if ! ../short infile /NoMatch/ 2>/dev/null; then
    echo "Test 21 PASSED"
fi
rm -f xx*

# Test for option --suppress-matched
../short -s infile /Line\ 3/ > /dev/null
test_content $'Line 1\nLine 2\n' xx00 && echo "Test 22 PASSED"
rm -f xx*

# Test max suffix length 
if ! ../short -n 1000000000000000000000 infile 3 2>/dev/null; then
    echo "Test 23 PASSED"
fi
rm -f xx*

# # Test too long name
# if ! ../short -n 100000000000000 infile 3 2>/dev/null; then
#     echo "Too long name length test PASSED"
# fi

# Test invalid flags and options
if ! ../short -z infile 3 2>/dev/null; then
    echo "Test 24 PASSED"
fi
rm -f xx*

# test bad repitition count 
if ! ../short infile 2 "{-1}" 2>/dev/null; then
    echo "Test 25 PASSED"
fi
rm -f xx*

# test non existent file
if ! ../short non_existent_file 2 2>/dev/null; then
    echo "Test 26 PASSED"
fi

# trigger ENAMETOOLONG error
long_prefix=$(printf '%*s' 255 | tr ' ' 'a')

# Attempt to use csplit with a long prefix. Adjust the '2' if necessary to control the suffix length.
if ! ../short -f="$long_prefix" infile 2>/dev/null; then
    echo "Test 27 PASSED"
fi

# Trigger overflow
if ! ../short infile 1000000000000000000000 2>/dev/null; then
    echo "Test 28 PASSED"
fi

# invalid regex pattern
if ! ../short infile /Line\ 2 2>/dev/null; then
    echo "Test 29 PASSED"
fi

# trigger invalid 
if ! ../short infile \\/Line\ 2 2>/dev/null; then
    echo "Test 30 PASSED"
fi

# trigger invalid 
if ! ../short infile %%/Line\ 2 2>/dev/null; then
    echo "Test 31 PASSED"
fi

# test invalid line number
if ! ../short infile -1 2>/dev/null; then
    echo "Test 32 PASSED"
fi

# Exiting
cleanup
cd ..
echo "All tests done."

# Code coverage
gcov ./src/main.rs
