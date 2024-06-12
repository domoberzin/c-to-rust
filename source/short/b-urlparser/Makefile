# compile urlparser_lib_test.c 
all: urlparser_lib_test_dbg urlparser_lib_test_opt urlparser_lib_test

urlparser_lib_test: urlparser_lib_test.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o urlparser_lib_test urlparser_lib_test.c

urlparser_lib_test_dbg: urlparser_lib_test.c
	gcc -g -O0 -o urlparser_lib_test_dbg urlparser_lib_test.c

urlparser_lib_test_opt: urlparser_lib_test.c
	gcc -O3 -o urlparser_lib_test_opt urlparser_lib_test.c

clean:
	rm -f urlparser_lib_test_dbg urlparser_lib_test urlparser_lib_test_opt *.gcda *.gcno *.gcov