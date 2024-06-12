#!/bin/bash

# Path to the printf program
PRINTF_PROGRAM="./printf"

# clean coverage files
rm -f *.gcda *.c.gcov

# Helper function for running a single test case
run_test() {
    local test_description="$1"
    local expected="$2"
    shift 2
    local output=$($PRINTF_PROGRAM "$@" 2>&1)
    if [[ "$output" == "$expected" ]]; then
        echo "PASS: $test_description"
    else
        echo "FAIL: $test_description - Expected '$expected', got '$output'"
    fi
}

# Basic format specifiers
run_test "Simple string" "hello world" "%s" "hello world"
run_test "Decimal integer" "123" "%d" "123"
run_test "Hexadecimal integer" "7b" "%x" "123"
run_test "Floating point" "123.456000" "%f" "123.456"

# Field widths and precision
run_test "Field width for integer" "  123" "%5d" "123"
run_test "Precision for floating point" "123.46" "%.2f" "123.456"
run_test "Combined field width and precision for float" " 123.46" "%7.2f" "123.456"

# Escape sequences
run_test "Newline escape" "Line 1"$'\n'"Line 2" "%s\n%s" "Line 1" "Line 2"
run_test "Tab escape" "Column 1"$'\t'"Column 2" "%s\t%s" "Column 1" "Column 2"

# Special characters and formats
run_test "Print percent sign" "%" "%%"
run_test "Octal to binary conversion" "A" "\\101"
run_test "Hexadecimal to binary conversion" "A" "\\x41"

# Field width and precision as arguments
run_test "Field width as argument" " hello" "%*s" "6" "hello"
run_test "Precision as argument for float" "3.14" "%.2f" "3.14159"

# Complex format strings
run_test "Complex string with multiple formats" "Number: 42, Float: 3.14, String: test" "Number: %d, Float: %.2f, String: %s" "42" "3.14" "test"

# Error cases
run_test "Missing format character" "printf: %: invalid directive" "%"
run_test "Invalid directive" "printf: %z: invalid directive" "%z" "123"

# Testing edge cases
run_test "Long integer" "9223372036854775807" "%d" "9223372036854775807"
run_test "Negative integer" "-123" "%d" "-123"
run_test "Large float" "1234567890.123456" "%f" "1234567890.123456"

# Code coverage
gcov ./printf.c