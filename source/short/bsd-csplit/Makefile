# compile csplit.c 
all: csplit_dbg csplit_opt csplit

csplit: csplit.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o csplit csplit.c

csplit_dbg: csplit.c
	gcc -g -O0 -o csplit_dbg csplit.c

csplit_opt: csplit.c
	gcc -O3 -o csplit_opt csplit.c

clean:
	rm -f csplit_dbg csplit csplit_opt *.gcda *.gcno *.gcov