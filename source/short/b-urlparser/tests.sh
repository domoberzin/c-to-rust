#!/bin/bash

# clean coverage files
rm -f *.gcda *.c.gcov

./urlparser_lib_test

# Coverage
gcov urlparser_lib_test.c
