#!/bin/bash

# Path to your expr program
EXPR_PROGRAM="./expr"

# clean coverage files
rm -f *.gcda *.c.gcov

# Helper function for running a single test case
run_test() {
    local test_description="$1"
    local expected="$2"
    shift 2
    local output=$("$EXPR_PROGRAM" "$@" 2>&1)
    if [[ "$output" == "$expected" ]]; then
        echo "PASS: $test_description"
    else
        echo "FAIL: $test_description - Expected '$expected', got '$output'"
    fi
}

# Arithmetic operations
run_test "Adding two numbers" "4" "2" "+" "2"
run_test "Subtracting two numbers" "0" "2" "-" "2"
run_test "Multiplying two numbers" "4" "2" "*" "2"
run_test "Dividing two numbers" "1" "2" "/" "2"
run_test "Modulo operation" "0" "2" "%" "2"

# Division by zero
run_test "Division by zero" "expr: division by zero" "2" "/" "0"

# Overflow (example test, might need adjustment based on system and program)
run_test "Multiplication overflow" "expr: overflow" "9223372036854775807" "*" "2"

# Comparisons
run_test "Equal numbers" "1" "2" "=" "2"
run_test "Not equal numbers" "0" "2" "=" "3"
run_test "Less than" "1" "2" "<" "3"
run_test "Greater than" "0" "2" ">" "3"

# Logical operations
run_test "AND operation (true && false)" "0" "1" "&" "0"
run_test "OR operation (false || true)" "1" "0" "|" "1"

# String operations and regex
run_test "String comparison (equal)" "1" "abc" "=" "abc"
run_test "String comparison (not equal)" "0" "abc" "=" "def"
# Regex match - assuming regex support (syntax might differ based on implementation)
# run_test "Regex match" "1" "hello" ":.*ello"

# Edge cases
run_test "Empty input" "" ""
run_test "Invalid operation" "expr: syntax error" "2" "?" "2"

# More tests
run_test "Regex match success" "5" "hello" ":" ".*ello"
run_test "Regex no match" "0" "hello" ":" "world"
run_test "Regex with group match" "0" "123abc456" ":" "([a-z]+)"

run_test "Unmatched right parenthesis" "expr: syntax error" ")" "+" "2"

run_test "Modulo by zero" "expr: division by zero" "2" "%" "0"

run_test "Addition overflow" "expr: overflow" "9223372036854775807" "+" "1"
run_test "Subtraction overflow" "expr: overflow" "-9223372036854775808" "-" "1"
run_test "Multiplication overflow" "expr: overflow" "4611686018427387904" "*" "2"

run_test "Invalid operand for addition" "expr: number \"abc\" is invalid" "abc" "+" "1"
run_test "Invalid operand for multiplication" "expr: number \"def\" is invalid" "1" "*" "def"

# Code coverage
gcov ./expr.c