# compile fmt.c 
all: fmt_dbg fmt_opt fmt

fmt: fmt.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o fmt fmt.c

fmt_dbg: fmt.c
	gcc -g -O0 -o fmt_dbg fmt.c

fmt_opt: fmt.c
	gcc -O3 -o fmt_opt fmt.c

clean:
	rm -f fmt_dbg fmt fmt_opt *.gcda *.gcno *.gcov