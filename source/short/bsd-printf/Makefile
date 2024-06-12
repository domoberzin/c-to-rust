# compile printf.c 
all: printf_dbg printf_opt printf

printf: printf.c
	gcc -O0 -fprofile-arcs -ftest-coverage -o printf printf.c

printf_dbg: printf.c
	gcc -g -O0 -o printf_dbg printf.c

printf_opt: printf.c
	gcc -O3 -o printf_opt printf.c

clean:
	rm -f printf_dbg printf printf_opt *.gcda *.gcno *.gcov