#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

# A function to log test results
# Now takes an expected return code as the second argument
log_result() {
  if [ $1 -eq $2 ]; then
    echo "PASS: $3"
  else
    echo "FAIL: $3"
  fi
}

# Create a temporary directory for the tests
TMP_DIR=$(mktemp -d)
cp ./test $TMP_DIR
cd $TMP_DIR || exit

# Test setup for file-based tests
setup_files() {
  echo "Setting up files..."
  echo "Hello, World!" > file1.txt
  echo "Just another test file." > file2.txt
  mkdir dir1
  ln -s file1.txt symlink1
  touch emptyfile
  mkfifo pipe1
}

# Cleanup the setup
cleanup() {
  echo "Cleaning up..."
  rm -rf "$TMP_DIR"
}

# Define test functions for different aspects of the utility

test_file_existence() {
  ./test -e file1.txt
  log_result $? 0 "file existence (file1.txt should exist)"
  
  ./test -e nosuchfile
  # Expect non-zero return code for failure, assuming 1 for simplicity
  log_result $? 1 "file existence (nosuchfile should not exist)"
}

test_file_type_checks() {
  ./test -f file1.txt
  log_result $? 0 "regular file check (file1.txt)"
  
  ./test -d dir1
  log_result $? 0 "directory check (dir1)"
  
  ./test -h symlink1
  log_result $? 0 "symlink check (symlink1)"
  
  ./test -p pipe1
  log_result $? 0 "named pipe check (pipe1)"
}

test_file_permissions() {
  chmod 644 file1.txt
  ./test -r file1.txt
  log_result $? 0 "read permission (file1.txt)"
  
  ./test -w file1.txt
  log_result $? 0 "write permission (file1.txt)"
  
  chmod +x file1.txt
  ./test -x file1.txt
  log_result $? 0 "execute permission (file1.txt)"
}

test_string_and_integer_comparisons() {
  ./test file1.txt = file1.txt
  log_result $? 0 "string equality (file1.txt = file1.txt)"
  
  ./test file1.txt != file2.txt
  log_result $? 0 "string inequality (file1.txt != file2.txt)"
  
  ./test 100 -eq 100
  log_result $? 0 "integer equality (100 -eq 100)"
  
  ./test 200 -gt 100
  log_result $? 0 "integer greater than (200 -gt 100)"
  
  ./test 50 -lt 100
  log_result $? 0 "integer less than (50 -lt 100)"
}

# Main test execution
setup_files

test_file_existence
test_file_type_checks
test_file_permissions
test_string_and_integer_comparisons

cleanup

cd - || exit

# Code coverage
gcov ./test.c
