#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

# Function to create temporary test files
create_temp_files() {
    # File 1
    echo -e "a 1\nb 2\nc 3" > file1.txt
    # File 2
    echo -e "a 1\nb 4\nd 4" > file2.txt
}

# Function to clean up test files
cleanup() {
    rm -f file1.txt file2.txt out.txt
}

# A function to assert equality
assert_equals() {
    if [[ "$1" != "$2" ]]; then
        echo "Test failed: Expected '$1' but got '$2'"
    else
        echo "Test PASS"
    fi
}

# Test default behavior (no options)
test_default_behavior() {
    create_temp_files
    ./join file1.txt file2.txt > out.txt
    assert_equals $'a 1 1\nb 2 4' "$(cat out.txt)" 
    cleanup
}

# Test -1 and -2 options
test_field_specification() {
    create_temp_files
    echo -e "1 a\n2 b\n3 c" > file1.txt
    echo -e "1 a\n4 b\n4 d" > file2.txt
    ./join -1 2 -2 2 file1.txt file2.txt > out.txt
    assert_equals $'a 1 1\nb 2 4' "$(cat out.txt)" 
    cleanup
}

# Test -a option
test_unpairable_lines() {
    create_temp_files
    ./join -a 1 file1.txt file2.txt > out.txt
    assert_equals $'a 1 1\nb 2 4\nc 3' "$(cat out.txt)" 
    cleanup
}

# Test -v option
test_exclusive_lines() {
    create_temp_files
    ./join -v 1 file1.txt file2.txt > out.txt
    assert_equals "c 3" "$(cat out.txt)" 
    cleanup
}

# Test -e option
test_empty_field_replacement() {
    create_temp_files
    ./join -e "NA" -a 1 file1.txt file2.txt > out.txt
    assert_equals $'a 1 1\nb 2 4\nc 3' "$(cat out.txt)"
    cleanup
}

# Test -t option
test_custom_delimiter() {
    echo -e "a,1\nb,2\nc,3" > file1.txt
    echo -e "a,1\nb,4\nd,4" > file2.txt
    ./join -t , file1.txt file2.txt > out.txt
    assert_equals $'a,1,1\nb,2,4' "$(cat out.txt)" 
    cleanup
}

# Test -o option
test_output_format() {
    create_temp_files
    ./join -o 0 1.1 2.2 file1.txt file2.txt > out.txt
    assert_equals $'a a 1\nb b 4' "$(cat out.txt)" 
    cleanup
}

# Run all tests
test_default_behavior
test_field_specification
test_unpairable_lines
test_exclusive_lines
test_empty_field_replacement
test_custom_delimiter
test_output_format

echo "All tests done."

# Code coverage
gcov ./join.c
