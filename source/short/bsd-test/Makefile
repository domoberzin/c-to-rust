# compile test.c 
all: test_dbg test_opt test

test: test.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o test test.c

test_dbg: test.c
	gcc -g -O0 -o test_dbg test.c

test_opt: test.c
	gcc -O3 -o test_opt test.c

clean:
	rm -f test_dbg test test_opt *.gcda *.gcno *.gcov