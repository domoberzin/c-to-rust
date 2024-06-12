#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

# Function to run a test case
# $1: Test description
# $2: Command to run
# $3: Expected output, with $'\n' for newline and $'\t' for tab
# $4: Input (optional)
run_test() {
  echo "Test: $1"
  if [ -z "$4" ]; then
    output=$($2)
  else
    output=$(echo -e "$4" | $2)
  fi
  if [ "$output" == "$3" ]; then
    echo " PASS"
  else
    echo " FAIL"
    echo " Expected: '$3'"
    echo " Got: '$output'"
  fi
}

# Test cases
run_test "Simple line break" "./fmt -w 10" $'This is a\ntest' "This is a test"
run_test "Preserve indentation" "./fmt -w 10 -p" $'    This\n    is a\n    test' "    This is a test"
run_test "Tab expansion" "./fmt -t 4" $'This is     a test' "This is\t\ta test"
run_test "Center text" "./fmt -w 20 -c" "   This is a test" "This is a test"
run_test "Mail header" "./fmt -m" $'Subject: Test\n\nHello' "Subject: Test\n\nHello"
run_test "No formatting" "./fmt -n" ".TH TEST" ".TH TEST"

echo "All tests done."

# Code coverage
gcov ./fmt.c
